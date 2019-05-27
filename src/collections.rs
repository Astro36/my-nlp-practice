use std::collections::HashMap;

pub struct Dictionary<'a> {
    indexes: std::collections::HashMap<&'a str, u32>,
    size: u32,
}

impl<'a> Dictionary<'a> {
    pub fn new() -> Dictionary<'a> {
        Dictionary {
            indexes: HashMap::new(),
            size: 0,
        }
    }

    pub fn from(tokens: &'a Vec<&'a str>) -> Dictionary {
        let mut indexes = HashMap::new();
        let mut max = 0;
        for token in tokens {
            if !indexes.contains_key(token) {
                max += 1;
                indexes.insert(*token, max);
            }
        }
        Dictionary { indexes, size: max }
    }

    pub fn extend(&mut self, tokens: &'a Vec<&'a str>) {
        let mut max = self.size;
        for token in tokens {
            if !self.indexes.contains_key(token) {
                max += 1;
                self.indexes.insert(*token, max);
            }
        }
        self.size = max;
    }

    pub fn get(&self, token: &'a str) -> u32 {
        match self.indexes.get(&token) {
            Some(index) => *index,
            None => 0,
        }
    }

    pub fn size(&self) -> usize {
        self.size as usize
    }
}

pub struct FrequencyDistribution<'a> {
    frequencies: std::collections::HashMap<&'a str, u32>,
}

impl<'a> FrequencyDistribution<'a> {
    pub fn new(tokens: &'a Vec<&'a str>) -> FrequencyDistribution {
        let mut frequencies = HashMap::new();
        for token in tokens {
            let frequency = frequencies.get(token).unwrap_or(&0) + 1;
            frequencies.insert(*token, frequency);
        }
        FrequencyDistribution { frequencies }
    }

    pub fn extend(&mut self, tokens: &'a Vec<&'a str>) {
        for token in tokens {
            let frequency = self.frequencies.get(token).unwrap_or(&0) + 1;
            self.frequencies.insert(*token, frequency);
        }
    }

    pub fn get(&self, token: &'a str) -> u32 {
        match self.frequencies.get(&token) {
            Some(frequency) => *frequency,
            None => 0,
        }
    }
}
