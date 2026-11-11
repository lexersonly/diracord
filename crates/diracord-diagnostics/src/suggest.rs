//! "Você quis dizer ...?" suggestions.

/// Edit distance between two strings (insertions, deletions, substitutions
/// and swaps of two neighbouring letters each cost 1), case-insensitive.
pub fn edit_distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().flat_map(char::to_lowercase).collect();
    let b: Vec<char> = b.chars().flat_map(char::to_lowercase).collect();
    let w = b.len() + 1;
    // Three rows are enough for the optimal-string-alignment variant.
    let mut rows = vec![0usize; 3 * w];
    for (j, cell) in rows[..w].iter_mut().enumerate() {
        *cell = j;
    }
    for i in 1..=a.len() {
        let (cur, prev, prev2) = ((i % 3) * w, ((i - 1) % 3) * w, ((i + 1) % 3) * w);
        rows[cur] = i;
        for j in 1..=b.len() {
            let cost = usize::from(a[i - 1] != b[j - 1]);
            let mut d = (rows[prev + j - 1] + cost)
                .min(rows[prev + j] + 1)
                .min(rows[cur + j - 1] + 1);
            if i > 1 && j > 1 && a[i - 1] == b[j - 2] && a[i - 2] == b[j - 1] {
                d = d.min(rows[prev2 + j - 2] + 1);
            }
            rows[cur + j] = d;
        }
    }
    rows[(a.len() % 3) * w + b.len()]
}

/// The candidate closest to `name`, if it is close enough to be a typo.
///
/// The allowed distance grows with the length of `name` (1 for 3–4 letters,
/// 2 for 5–8, 3 for longer). A difference only in upper/lower case always
/// matches. Ties keep the first candidate.
///
/// ```
/// use diracord_diagnostics::did_you_mean;
/// assert_eq!(did_you_mean("rpely", ["reply", "send", "ban"]), Some("reply"));
/// assert_eq!(did_you_mean("xyz", ["reply"]), None);
/// ```
pub fn did_you_mean<'a, I>(name: &str, candidates: I) -> Option<&'a str>
where
    I: IntoIterator<Item = &'a str>,
{
    let max = match name.chars().count() {
        0..=2 => 0,
        3..=4 => 1,
        5..=8 => 2,
        _ => 3,
    };
    let mut best: Option<(&str, usize)> = None;
    for c in candidates {
        if c == name {
            continue;
        }
        let d = edit_distance(name, c);
        if (d <= max || d == 0) && best.is_none_or(|(_, bd)| d < bd) {
            best = Some((c, d));
        }
    }
    best.map(|(c, _)| c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance() {
        assert_eq!(edit_distance("kitten", "sitting"), 3);
        assert_eq!(edit_distance("", "abc"), 3);
        assert_eq!(edit_distance("sned", "send"), 1);
        assert_eq!(edit_distance("Reply", "reply"), 0);
    }

    #[test]
    fn suggests_case_difference() {
        assert_eq!(
            did_you_mean("memberjoin", ["memberJoin"]),
            Some("memberJoin")
        );
    }

    #[test]
    fn short_names_need_exact_shape() {
        assert_eq!(did_you_mean("bn", ["ban"]), None);
        assert_eq!(did_you_mean("bam", ["ban"]), Some("ban"));
    }

    #[test]
    fn picks_closest() {
        assert_eq!(did_you_mean("sned", ["send", "seed"]), Some("send"));
    }
}
