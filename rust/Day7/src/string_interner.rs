use std::collections::HashMap;

pub type StringKey = u16;

#[derive(Default)]
pub struct StringInterner {
    pub lookup_to_key: HashMap<String, StringKey>,
    pub lookup_to_string: Vec<String>,
}

impl StringInterner {
    pub fn new() -> StringInterner {
        StringInterner {
            lookup_to_key: HashMap::new(),
            lookup_to_string: vec![],
        }
    }

    pub fn get_key_or_insert(&mut self, string: &str) -> StringKey {
        if let Some(key) = self.lookup_to_key.get(string) {
            *key
        } else {
            let owned = string.to_owned();
            self.lookup_to_string.push(owned);
            let key = (self.lookup_to_string.len() - 1) as StringKey;

            let owned = string.to_owned();
            self.lookup_to_key.insert(owned, key);
            key
        }
    }

    pub fn get_key(&self, string: &str) -> StringKey {
        self.lookup_to_key[string] as StringKey
    }

    pub fn lookup_string(&self, string: StringKey) -> &str {
        &self.lookup_to_string[string as usize]
    }
}
