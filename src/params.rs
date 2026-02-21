use std::collections::BTreeMap;

use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
#[serde(transparent)]
pub struct PveParams(pub BTreeMap<String, String>);

impl PveParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.0.insert(key.into(), value.into());
    }

    pub fn with<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.insert(key, value);
        self
    }

    pub fn insert_opt<K, V>(&mut self, key: K, value: Option<V>)
    where
        K: Into<String>,
        V: Into<String>,
    {
        if let Some(value) = value {
            self.insert(key, value);
        }
    }

    pub fn insert_bool<K>(&mut self, key: K, value: bool)
    where
        K: Into<String>,
    {
        self.insert(key, if value { "1" } else { "0" });
    }

    pub fn with_bool<K>(mut self, key: K, value: bool) -> Self
    where
        K: Into<String>,
    {
        self.insert_bool(key, value);
        self
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn extend(&mut self, other: &PveParams) {
        for (key, value) in &other.0 {
            self.0.insert(key.clone(), value.clone());
        }
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.0.remove(key)
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(String::as_str)
    }

    pub fn into_inner(self) -> BTreeMap<String, String> {
        self.0
    }
}

impl<K, V> FromIterator<(K, V)> for PveParams
where
    K: Into<String>,
    V: Into<String>,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut params = Self::new();
        for (k, v) in iter {
            params.insert(k, v);
        }
        params
    }
}
