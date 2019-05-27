use super::collections::{Dictionary, FrequencyDistribution};

pub struct TDM {
    pub documents: Vec<Vec<u32>>,
}

impl TDM {
    pub fn from(documents_tokens: &Vec<Vec<&str>>) -> TDM {
        let mut indexes = Dictionary::new();
        documents_tokens
            .iter()
            .for_each(|document_tokens| indexes.extend(document_tokens));
        let documents = documents_tokens
            .iter()
            .map(|document_tokens| {
                let freq_dist = FrequencyDistribution::new(&document_tokens);
                let mut document = vec![0; 10];
                document_tokens.iter().for_each(|token| {
                    document[indexes.get(token) as usize] = freq_dist.get(token);
                });
                document
            })
            .collect();
        TDM { documents }
    }
}
