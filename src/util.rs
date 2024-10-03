pub mod hamming {
    use crate::xor::{xor, XorError};
    use thiserror::Error;

    #[derive(Error, Debug)]
    enum HammingError {
        #[error("length mismatch {0}, {1}")]
        InvalidLength(usize, usize),
    }

    /// popcount over a ^ b words of the same len
    pub fn distance(a: &[u8], b: &[u8]) -> Result<usize, HammingError> {
        // only hamming distance
        if a.len() != b.len() {
            return Err(HammingError::InvalidLength(a.len(), b.len()));
        }
        let xored_bytes = xor(a, b).unwrap(); // safe unwrap, len is the same
        Ok(xored_bytes.iter().fold(0, |mut acc, b| {
            acc += b.count_ones() as usize;
            acc
        }))
    }

    #[cfg(test)]
    mod tests {
        use super::distance;

        #[test]
        fn test_hamming_distance() {
            let one = "this is a test";
            let two = "wokka wokka!!!";
            let three = "this is a test";
            assert_eq!(distance(one.as_bytes(), two.as_bytes()).unwrap(), 37);
            assert_eq!(distance(one.as_bytes(), three.as_bytes()).unwrap(), 0);
        }
    }
}
