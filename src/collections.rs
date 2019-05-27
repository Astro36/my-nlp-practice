use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Result};
use std::iter::FromIterator;

pub struct Dictionary<'a> {
    indexes: std::collections::HashMap<&'a str, u32>,
    size: u32,
}

impl<'a> Debug for Dictionary<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Dictionary {:#?}", self.indexes)
    }
}

impl<'a> Extend<&'a str> for Dictionary<'a> {
    fn extend<I: IntoIterator<Item = &'a str>>(&mut self, tokens: I) {
        let mut max = self.size;
        for token in tokens {
            if !self.indexes.contains_key(&token) {
                max += 1;
                self.indexes.insert(token, max);
            }
        }
        self.size = max;
    }
}

impl<'a> FromIterator<&'a str> for Dictionary<'a> {
    fn from_iter<I: IntoIterator<Item = &'a str>>(tokens: I) -> Dictionary<'a> {
        let mut indexes = HashMap::new();
        let mut max = 0;
        for token in tokens {
            if !indexes.contains_key(&token) {
                max += 1;
                indexes.insert(token, max);
            }
        }
        Dictionary { indexes, size: max }
    }
}


impl<'a> Dictionary<'a> {
    pub fn new() -> Dictionary<'a> {
        Dictionary {
            indexes: HashMap::new(),
            size: 0,
        }
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

impl<'a> Debug for FrequencyDistribution<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "FrequencyDistribution {:#?}", self.frequencies)
    }
}

impl<'a> Extend<&'a str> for FrequencyDistribution<'a> {
    fn extend<I: IntoIterator<Item = &'a str>>(&mut self, tokens: I) {
        for token in tokens {
            let frequency = self.frequencies.get(&token).unwrap_or(&0) + 1;
            self.frequencies.insert(token, frequency);
        }
    }
}

impl<'a> FromIterator<&'a str> for FrequencyDistribution<'a> {
    fn from_iter<I: IntoIterator<Item = &'a str>>(tokens: I) -> FrequencyDistribution<'a> {
        let mut frequencies = HashMap::new();
        for token in tokens {
            let frequency = frequencies.get(&token).unwrap_or(&0) + 1;
            frequencies.insert(token, frequency);
        }
        FrequencyDistribution { frequencies }
    }
}

impl<'a> FrequencyDistribution<'a> {
    pub fn new() -> FrequencyDistribution<'a> {
        FrequencyDistribution {
            frequencies: HashMap::new(),
        }
    }

    pub fn get(&self, token: &'a str) -> u32 {
        match self.frequencies.get(&token) {
            Some(frequency) => *frequency,
            None => 0,
        }
    }
}
