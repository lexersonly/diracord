//! File driver, format **v0 (provisional)**: an append-only log in one file.
//!
//! ```text
//! header : "DIRABANK" | version u16 = 0 | reserved u16
//! record : crc32 u32 | op u8 (1 = put, 2 = delete) | key_len u32 | value_len u32 | key | value
//! ```
//!
//! All integers are little-endian; the CRC covers everything after itself.
//! On open, records are scanned to rebuild an in-memory index of
//! `key → (offset, len)`; values stay on disk. A torn last record (power
//! loss mid-write) fails its CRC and is cut off. When more than half of the
//! file is old versions, it is rewritten (compacted) to a temporary file
//! that atomically replaces the original.

use crate::crc::crc32;
use crate::{Driver, DriverError};
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

const MAGIC: &[u8; 8] = b"DIRABANK";
const VERSION: u16 = 0;
const HEADER_LEN: u64 = 12;
const RECORD_HEAD: usize = 13;
const OP_PUT: u8 = 1;
const OP_DEL: u8 = 2;
const MAX_KEY: usize = 64 * 1024;
const MAX_VALUE: usize = 16 * 1024 * 1024;
/// Compact only files bigger than this.
const COMPACT_MIN: u64 = 1024 * 1024;

/// Size information about an open file.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FileStats {
    /// Number of live keys.
    pub keys: usize,
    /// Size of the file on disk.
    pub file_bytes: u64,
    /// Bytes used by live records (the rest is garbage until compaction).
    pub live_bytes: u64,
}

/// Single-file, append-only driver. RAM use is the index only.
#[derive(Debug)]
pub struct FileDriver {
    path: PathBuf,
    file: File,
    /// key → (offset of value, value length, record length)
    index: HashMap<Vec<u8>, (u64, u32, u32)>,
    end: u64,
    live: u64,
}

impl FileDriver {
    /// Opens or creates the file at `path`.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, DriverError> {
        let path = path.as_ref().to_path_buf();
        if let Some(dir) = path.parent().filter(|d| !d.as_os_str().is_empty()) {
            fs::create_dir_all(dir)?;
        }
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&path)?;
        let mut d = FileDriver {
            path,
            file,
            index: HashMap::new(),
            end: 0,
            live: 0,
        };
        d.load()?;
        Ok(d)
    }

    /// Size information.
    pub fn stats(&self) -> FileStats {
        FileStats {
            keys: self.index.len(),
            file_bytes: self.end,
            live_bytes: self.live + HEADER_LEN,
        }
    }

    fn load(&mut self) -> Result<(), DriverError> {
        let len = self.file.metadata()?.len();
        if len == 0 {
            let mut h = Vec::with_capacity(HEADER_LEN as usize);
            h.extend_from_slice(MAGIC);
            h.extend_from_slice(&VERSION.to_le_bytes());
            h.extend_from_slice(&0u16.to_le_bytes());
            self.file.write_all(&h)?;
            self.file.sync_data()?;
            self.end = HEADER_LEN;
            return Ok(());
        }
        let mut r = BufReader::new(&self.file);
        let mut h = [0u8; HEADER_LEN as usize];
        r.read_exact(&mut h)
            .map_err(|_| DriverError::BadFile("cabeçalho incompleto".into()))?;
        if &h[..8] != MAGIC {
            return Err(DriverError::BadFile("não é um arquivo do Dirabank".into()));
        }
        let version = u16::from_le_bytes([h[8], h[9]]);
        if version != VERSION {
            return Err(DriverError::BadFile(format!(
                "versão {version} não suportada (esta é a {VERSION})"
            )));
        }
        let mut pos = HEADER_LEN;
        let mut index = HashMap::new();
        let mut live = 0u64;
        while let Some(rec) = read_record(&mut r) {
            let value_at = pos + RECORD_HEAD as u64 + rec.key.len() as u64;
            if let Some((_, _, old)) = index.remove(&rec.key) {
                live -= old as u64;
            }
            if rec.op == OP_PUT {
                index.insert(rec.key, (value_at, rec.value_len, rec.len));
                live += rec.len as u64;
            }
            pos += rec.len as u64;
        }
        drop(r);
        if pos < len {
            // Torn or corrupt tail: keep everything before it.
            self.file.set_len(pos)?;
            self.file.sync_data()?;
        }
        self.index = index;
        self.end = pos;
        self.live = live;
        Ok(())
    }

    fn append(&mut self, op: u8, key: &[u8], value: &[u8]) -> Result<u32, DriverError> {
        if key.len() > MAX_KEY || value.len() > MAX_VALUE {
            return Err(DriverError::TooLarge);
        }
        let mut head = [0u8; RECORD_HEAD];
        head[4] = op;
        head[5..9].copy_from_slice(&(key.len() as u32).to_le_bytes());
        head[9..13].copy_from_slice(&(value.len() as u32).to_le_bytes());
        let crc = crc32(&[&head[4..], key, value]);
        head[..4].copy_from_slice(&crc.to_le_bytes());
        let mut rec = Vec::with_capacity(RECORD_HEAD + key.len() + value.len());
        rec.extend_from_slice(&head);
        rec.extend_from_slice(key);
        rec.extend_from_slice(value);
        self.file.seek(SeekFrom::Start(self.end))?;
        self.file.write_all(&rec)?;
        self.end += rec.len() as u64;
        Ok(rec.len() as u32)
    }

    fn maybe_compact(&mut self) -> Result<(), DriverError> {
        let garbage = self.end.saturating_sub(self.live + HEADER_LEN);
        if self.end > COMPACT_MIN && garbage > self.end / 2 {
            self.compact()?;
        }
        Ok(())
    }

    /// Rewrites the file with only the live values.
    pub fn compact(&mut self) -> Result<(), DriverError> {
        let tmp = self.path.with_extension("compact.tmp");
        let _ = fs::remove_file(&tmp);
        {
            let mut out = FileDriver::open(&tmp)?;
            let keys: Vec<Vec<u8>> = self.index.keys().cloned().collect();
            for k in keys {
                if let Some(v) = self.get(&k)? {
                    out.put(&k, &v)?;
                }
            }
            out.file.sync_all()?;
        }
        fs::rename(&tmp, &self.path)?;
        *self = FileDriver::open(&self.path)?;
        Ok(())
    }
}

/// A record found while scanning the file.
struct Scanned {
    op: u8,
    key: Vec<u8>,
    value_len: u32,
    /// Whole record length, header included.
    len: u32,
}

/// Reads one record. `None` at the end or at a torn/corrupt record.
fn read_record(r: &mut impl Read) -> Option<Scanned> {
    let mut head = [0u8; RECORD_HEAD];
    if r.read_exact(&mut head).is_err() {
        return None;
    }
    let crc = u32::from_le_bytes([head[0], head[1], head[2], head[3]]);
    let op = head[4];
    let klen = u32::from_le_bytes([head[5], head[6], head[7], head[8]]) as usize;
    let vlen = u32::from_le_bytes([head[9], head[10], head[11], head[12]]) as usize;
    if !(op == OP_PUT || op == OP_DEL) || klen > MAX_KEY || vlen > MAX_VALUE {
        return None;
    }
    let mut key = vec![0u8; klen];
    let mut value = vec![0u8; vlen];
    if r.read_exact(&mut key).is_err() || r.read_exact(&mut value).is_err() {
        return None;
    }
    if crc32(&[&head[4..], &key, &value]) != crc {
        return None;
    }
    Some(Scanned {
        op,
        key,
        value_len: vlen as u32,
        len: (RECORD_HEAD + klen + vlen) as u32,
    })
}

impl Driver for FileDriver {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, DriverError> {
        let Some(&(at, len, _)) = self.index.get(key) else {
            return Ok(None);
        };
        let mut f = &self.file;
        f.seek(SeekFrom::Start(at))?;
        let mut buf = vec![0u8; len as usize];
        f.read_exact(&mut buf)?;
        Ok(Some(buf))
    }

    fn put(&mut self, key: &[u8], value: &[u8]) -> Result<(), DriverError> {
        let start = self.end;
        let rec_len = self.append(OP_PUT, key, value)?;
        if let Some((_, _, old)) = self.index.insert(
            key.to_vec(),
            (
                start + RECORD_HEAD as u64 + key.len() as u64,
                value.len() as u32,
                rec_len,
            ),
        ) {
            self.live -= old as u64;
        }
        self.live += rec_len as u64;
        self.maybe_compact()
    }

    fn remove(&mut self, key: &[u8]) -> Result<bool, DriverError> {
        let Some((_, _, old)) = self.index.remove(key) else {
            return Ok(false);
        };
        self.live -= old as u64;
        self.append(OP_DEL, key, &[])?;
        self.maybe_compact()?;
        Ok(true)
    }

    fn flush(&mut self) -> Result<(), DriverError> {
        self.file.sync_data()?;
        Ok(())
    }
}
