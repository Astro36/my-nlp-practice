use super::collections::Dictionary;
use std::iter::FromIterator;

pub fn encode_int(tokens: Vec<&str>) -> Vec<u32> {
    let indexes = Dictionary::from_iter(tokens.clone().into_iter());
    tokens.iter().map(|token| indexes.get(*token)).collect()
}

pub fn encode_one_hot(tokens: Vec<&str>) -> Vec<Vec<u32>> {
    let ints = encode_int(tokens);
    let max = *ints.iter().max().unwrap();
    let len = max as usize + 1;
    ints.into_iter()
        .map(|i| {
            let mut output = vec![0; len];
            output[i as usize] = 1;
            output
        })
        .collect()
}

pub fn tokenize(text: &str) -> Vec<&str> {
    text.split_whitespace().collect()
}
