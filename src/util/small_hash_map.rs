#[derive(Debug)]
pub struct SmallMap<K, V>(Vec<(K, V)>);

impl<K, V> Default for SmallMap<K, V> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<K, V> SmallMap<K, V> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_capacity(size: usize) -> Self {
        Self(Vec::with_capacity(size))
    }
}

impl<K, V> SmallMap<K, V>
where
    K: PartialEq,
{
    pub fn insert(&mut self, k: K, v: V) -> bool {
        if self.0.iter().any(|e| e.0 == k) {
            false
        } else {
            self.0.push((k, v));
            true
        }
    }
}

pub struct SmallSet<K>(SmallMap<K, ()>);

impl<K> SmallSet<K> {
    #[inline]
    pub fn new() -> Self {
        Self(Default::default())
    }

    #[inline]
    pub fn with_capacity(size: usize) -> Self {
        Self(SmallMap::with_capacity(size))
    }
}

impl<K> Set<K, ()> for SmallSet<K>
where
    K: PartialEq,
{
    #[inline]
    fn insert(&mut self, k: K) -> bool {
        self.0.insert(k, ())
    }
}

pub trait Set<K, H> {
    fn insert(&mut self, k: K) -> bool;
}

impl<K, H> Set<K, H> for std::collections::HashSet<K, H>
where
    H: std::hash::BuildHasher,
    K: Eq + std::hash::Hash,
{
    #[inline]
    fn insert(&mut self, k: K) -> bool {
        std::collections::HashSet::<K, H>::insert(self, k)
    }
}
