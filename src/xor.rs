use thiserror::Error;
#[derive(Debug, Error, PartialEq, Eq)]
pub enum XorError {
    #[error("xor inputs mismatch input len: {0}, key len: {0}")]
    LengthMismatch(usize, usize),
}
//
pub fn xor(input: &[u8], key: &[u8]) -> Result<Vec<u8>, XorError> {
    if input.len() != key.len() {
        return Err(XorError::LengthMismatch(input.len(), key.len()));
    }

    Ok(input.iter().zip(key.iter()).map(|(i, k)| i ^ k).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn len_mismatch() {
        let input = vec![0, 0, 1, 0, 2];
        let key = vec![0, 1, 0, 2];
        let result = xor(&input, &key);

        assert_eq!(result, Err(XorError::LengthMismatch(5, 4)));
    }
}
