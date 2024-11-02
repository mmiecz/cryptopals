use std::collections::HashMap;

//English letter frequencies
const ENGLISH_FREQ: &[(char, f64)] = &[
    ('e', 12.70),
    ('t', 9.06),
    ('a', 8.17),
    ('o', 7.51),
    ('i', 6.97),
    ('n', 6.75),
    ('s', 6.33),
    ('h', 6.09),
    ('r', 5.99),
    ('d', 4.25),
    ('l', 4.03),
    ('c', 2.78),
    ('u', 2.76),
    ('m', 2.41),
    ('w', 2.36),
    ('f', 2.23),
    ('g', 2.02),
    ('y', 1.97),
    ('p', 1.93),
    ('b', 1.29),
    ('v', 0.98),
    ('k', 0.77),
    ('j', 0.15),
    ('x', 0.15),
    ('q', 0.10),
    ('z', 0.07),
];

/// Calculates likehood of text being english by performing letter freq analysis
pub fn english_likeness(text: &str) -> f64 {
    if text.is_empty() {
        return 0.0;
    }

    let mut letters_count: HashMap<char, usize> = HashMap::new();
    let mut count = 0;

    for c in text.chars().filter(|c| c.is_alphabetic()) {
        letters_count.entry(c).and_modify(|e| *e += 1).or_insert(1);
        count += 1;
    }

    // none of the letters in the text is alphabetic, can't be english text
    if count == 0 {
        return 0.0;
    }

    // find how well found freq matches english letter freq in text
    ENGLISH_FREQ
        .iter()
        .map(|(c, freq)| {
            let letter_freq: f64 =
                *letters_count.get(c).unwrap_or(&0) as f64 / count as f64 * 100.0; // percentage of given letter found in text
            1.0 - (letter_freq - freq).abs() / 100.0 // how well does this fit with theorethical freqs?
        })
        .sum::<f64>()
        / ENGLISH_FREQ.len() as f64
}

#[cfg(test)]
mod tests {
    use crate::lang_detection::english_likeness;

    #[test]
    fn test_perfect_match() {
        let text = "the quick brown fox jumps over the lazy dog";
        assert!(english_likeness(text) > 0.95);
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(english_likeness(""), 0.0);
    }

    #[test]
    fn proper_english_better_than_partial() {
        let english = "The quick brown fox jumps over the lazy dog";
        let partial_english = "The qck brwn fx jmps ver the lzy dog";

        let proper = english_likeness(english);
        let partial = english_likeness(partial_english);

        assert!(proper > partial);
    }

    #[test]
    fn none_of_the_letters_is_alphabetic() {
        let b = &[0u8, 0, 0, 0];
        let s = String::from_utf8_lossy(b);
        let score = english_likeness(&s);
        assert_eq!(score, 0.0);
    }
}
