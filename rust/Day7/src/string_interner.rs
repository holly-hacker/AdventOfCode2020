use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Default)]
pub struct StringKey(u16);

impl StringKey {
    // TODO: bad! this should not be allowed!
    pub fn from(data: u16) -> Self {
        StringKey(data)
    }

    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }
}

#[derive(Default)]
pub struct StringInterner {
    pub map: HashMap<String, StringKey>,
    pub vec: Vec<String>,
}

impl StringInterner {
    pub fn intern(&mut self, string: &str) -> StringKey {
        if let Some(key) = self.map.get(string) {
            *key
        } else {
            let owned = string.to_owned();
            self.vec.push(owned);
            let key = StringKey((self.vec.len() - 1) as u16);

            let owned = string.to_owned();
            self.map.insert(owned, key);
            key
        }
    }

    pub fn get_key(&self, string: &str) -> StringKey {
        self.map[string] as StringKey
    }

    #[allow(unused)]
    pub fn lookup(&self, string: StringKey) -> &str {
        &self.vec[string.0 as usize]
    }
}
