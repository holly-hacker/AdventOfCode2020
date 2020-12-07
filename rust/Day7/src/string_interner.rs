use std::collections::HashMap;

#[derive(Default)]
pub struct StringInterner {
    pub lookup_to_key: HashMap<String, usize>,
    pub lookup_to_string: Vec<String>,
}

impl StringInterner {
    pub fn new() -> StringInterner {
        StringInterner {
            lookup_to_key: HashMap::new(),
            lookup_to_string: vec![],
        }
    }

    pub fn get_key_or_insert(&mut self, string: &str) -> usize {
        if let Some(key) = self.lookup_to_key.get(string) {
            *key
        } else {
            let owned = string.to_owned();
            self.lookup_to_string.push(owned);
            let key = self.lookup_to_string.len() - 1;

            let owned = string.to_owned();
            self.lookup_to_key.insert(owned, key);
            key
        }
    }

    pub fn get_key(&self, string: &str) -> usize {
        self.lookup_to_key[string]
    }

    pub fn lookup_string(&self, string: usize) -> &str {
        &self.lookup_to_string[string]
    }
}
