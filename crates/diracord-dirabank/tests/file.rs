//! File driver: persistence, crash recovery, compaction.

use diracord_dirabank::{Dirabank, Driver, DriverError, FileDriver};
use diracord_value::{Scope, Store, StoreKey, Value};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

fn temp(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("dirabank-test-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir.join("data.dirabank")
}

fn key(name: &str) -> StoreKey {
    StoreKey::new(Scope::User, "42", name)
}

#[test]
fn values_survive_reopen() {
    let path = temp("reopen");
    {
        let mut db = Dirabank::open(&path).unwrap();
        db.set(&key("coins"), Value::Int(5)).unwrap();
        db.set(&key("coins"), Value::Int(6)).unwrap();
        db.set(&key("name"), Value::from("Ana")).unwrap();
        db.set(&key("tmp"), Value::Bool(true)).unwrap();
        assert!(db.delete(&key("tmp")).unwrap());
        db.flush().unwrap();
    }
    let db = Dirabank::open(&path).unwrap();
    assert_eq!(db.get(&key("coins")).unwrap(), Some(Value::Int(6)));
    assert_eq!(db.get(&key("name")).unwrap(), Some(Value::from("Ana")));
    assert_eq!(db.get(&key("tmp")).unwrap(), None);
    assert_eq!(db.driver().stats().keys, 2);
}

#[test]
fn torn_tail_is_cut_off() {
    let path = temp("torn");
    {
        let mut db = Dirabank::open(&path).unwrap();
        db.set(&key("a"), Value::Int(1)).unwrap();
    }
    let good_len = std::fs::metadata(&path).unwrap().len();
    // Half a record, as if power was lost mid-write.
    OpenOptions::new()
        .append(true)
        .open(&path)
        .unwrap()
        .write_all(&[1, 2, 3, 4, 1, 9, 0])
        .unwrap();
    let mut db = Dirabank::open(&path).unwrap();
    assert_eq!(db.get(&key("a")).unwrap(), Some(Value::Int(1)));
    assert_eq!(std::fs::metadata(&path).unwrap().len(), good_len);
    db.set(&key("b"), Value::Int(2)).unwrap();
    drop(db);
    assert_eq!(
        Dirabank::open(&path).unwrap().get(&key("b")).unwrap(),
        Some(Value::Int(2))
    );
}

#[test]
fn rejects_other_files() {
    let path = temp("other");
    std::fs::write(&path, b"not a database at all").unwrap();
    assert!(matches!(
        FileDriver::open(&path),
        Err(DriverError::BadFile(_))
    ));
}

#[test]
fn compaction_keeps_live_values() {
    let path = temp("compact");
    let mut db = Dirabank::open(&path).unwrap();
    let big = Value::from("x".repeat(4096));
    for i in 0..600 {
        db.set(&key("big"), big.clone()).unwrap();
        db.set(&key("n"), Value::Int(i)).unwrap();
    }
    let stats = db.driver().stats();
    assert!(
        stats.file_bytes < 2 * 1024 * 1024,
        "não compactou: {stats:?}"
    );
    assert_eq!(db.get(&key("n")).unwrap(), Some(Value::Int(599)));
    assert_eq!(db.get(&key("big")).unwrap(), Some(big.clone()));
    drop(db);
    assert_eq!(
        Dirabank::open(&path).unwrap().get(&key("n")).unwrap(),
        Some(Value::Int(599))
    );
}

#[test]
fn explicit_compact() {
    let path = temp("explicit");
    let mut d = FileDriver::open(&path).unwrap();
    for i in 0..10u8 {
        d.put(b"k", &[i]).unwrap();
    }
    let before = d.stats().file_bytes;
    d.compact().unwrap();
    assert!(d.stats().file_bytes < before);
    assert_eq!(d.get(b"k").unwrap(), Some(vec![9]));
}

#[test]
fn memory_driver_works_as_store() {
    let mut db = Dirabank::memory();
    db.set(&key("a"), Value::List(vec![1i64.into()])).unwrap();
    assert_eq!(
        db.get(&key("a")).unwrap(),
        Some(Value::List(vec![1i64.into()]))
    );
    assert!(!db.delete(&key("zzz")).unwrap());
}

#[test]
fn new_files_are_v1_and_v0_files_are_upgraded() {
    let path = temp("v0");
    {
        let mut db = Dirabank::open(&path).unwrap();
        db.set(&key("coins"), Value::Int(3)).unwrap();
    }
    let bytes = std::fs::read(&path).unwrap();
    assert_eq!(&bytes[..10], b"DIRABANK\x01\x00", "arquivos novos são v1");
    // Simulate a file written by the provisional v0.
    let mut old = bytes.clone();
    old[8] = 0;
    std::fs::write(&path, &old).unwrap();
    let db = Dirabank::open(&path).unwrap();
    assert_eq!(db.get(&key("coins")).unwrap(), Some(Value::Int(3)));
    drop(db);
    assert_eq!(
        std::fs::read(&path).unwrap(),
        bytes,
        "v0 vira v1 sem mudar mais nada"
    );
    let mut future = bytes;
    future[8] = 9;
    std::fs::write(&path, &future).unwrap();
    assert!(
        matches!(FileDriver::open(&path), Err(DriverError::BadFile(m)) if m.contains("versão 9"))
    );
}
