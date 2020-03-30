use std::collections::HashMap;

pub struct Scope<'a, K, V>
where
    K: std::cmp::Eq + std::hash::Hash
{
    parent: Option<&'a Scope<'a,K, V>>,
    map: HashMap<K, V>
}

impl<'a, K, V> Scope<'a, K, V>
where
    K: std::cmp::Eq + std::hash::Hash
{
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        match self.map.get_mut(key) {
            some => some,
            None => if let Some(scope) = self.parent {
                scope.get_mut(key)
            } else {
                None
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        match self.map.get(key) {
            some => some,
            None => if let Some(scope) = self.parent {
                scope.get(key)
            } else {
                None
            }
        }
    }

    pub fn has(&self, key: &K) -> bool {
        match self.map.get(key) {
            Some(_) => true,
            None => if let Some(scope) = self.parent {
                scope.has(key)
            } else {
                false
            }
        }
    }
}
