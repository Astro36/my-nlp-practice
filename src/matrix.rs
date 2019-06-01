use super::collections::{Dictionary, FrequencyDistribution};
use std::fmt::{Debug, Formatter, Result};
use std::iter::FromIterator;

pub struct TDM {
    pub keys: Vec<String>,
    pub values: Vec<Vec<u32>>,
}

impl Debug for TDM {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "TDM [\n\t{:?},\n{}]",
            self.keys,
            self.values
                .iter()
                .fold(String::new(), |acc, v| acc + &format!("\t{:?},\n", v))
        )
    }
}

impl<'a> From<&'a Vec<Vec<&str>>> for TDM {
    fn from(documents_tokens: &'a Vec<Vec<&str>>) -> Self {
        let mut indexes = Dictionary::new();
        documents_tokens
            .clone()
            .into_iter()
            .for_each(|document_tokens| indexes.extend(document_tokens.into_iter()));
        let keys = indexes.keys();
        let values = documents_tokens
            .iter()
            .map(|document_tokens| {
                let freq_dist =
                    FrequencyDistribution::from_iter(document_tokens.clone().into_iter());
                let mut document = vec![0; 10];
                document_tokens.iter().for_each(|token| {
                    document[indexes.get(token) as usize] = freq_dist.get(token);
                });
                document
            })
            .collect();
        TDM { keys, values }
    }
}

pub struct TfidfMatrix {
    pub keys: Vec<String>,
    pub values: Vec<Vec<f64>>,
}

impl Debug for TfidfMatrix {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "TfidfMatrix [\n\t{:?},\n{}]",
            self.keys,
            self.values
                .iter()
                .fold(String::new(), |acc, v| acc + &format!("\t{:?},\n", v))
        )
    }
}

impl<'a> From<&'a TDM> for TfidfMatrix {
    fn from(tdm: &'a TDM) -> Self {
        let n = tdm.values.len();
        let keys = tdm.keys.clone();
        let values = tdm
            .values
            .iter()
            .map(|document| {
                document
                    .iter()
                    .enumerate()
                    .map(|(index, term)| {
                        let tf = *term as f64;
                        let df = tdm.values.iter().filter(|d| d[index as usize] > 0).count() as f64;
                        let idf = ((n + 1) as f64 / (1.0 + df)).ln();
                        let tfidf = tf * idf;
                        tfidf
                    })
                    .collect()
            })
            .collect();
        TfidfMatrix { keys, values }
    }
}
