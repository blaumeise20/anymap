/// HashMap with any type for values like JavaScript objects.
/// 
/// # Examples
/// ```
/// use anymap::AnyMap;
/// 
/// let mut map = AnyMap::new();
/// map.insert("key", "value");
/// map.insert("key2", 1);
/// assert_eq!(map.get_typed::<&str>("key").unwrap(), "value");
/// assert_eq!(map.get_typed::<i32>("key2").unwrap(), 1);
/// ```
/// 
/// ```
/// use anymap::{AnyMap};
/// 
/// let mut map = AnyMap::new();
/// map.insert("key", "value");
/// assert!(map.get("key").unwrap().is::<&str>());
/// ```

use std::{any::Any, collections::{HashMap, hash_map::{Keys, Values, Iter}}, borrow::Borrow};
use core::hash::Hash;

#[derive(Debug)]
pub struct Value {
    inner: Box<dyn Any>
}

impl Value {
    pub(crate) fn new<T: Any>(value: T) -> Self {
        Self {
            inner: Box::new(value),
        }
    }

    pub fn as_type<T: Any>(&self) -> Option<&T> {
        (*self.inner).downcast_ref::<T>()
    }

    pub fn is<T: Any>(&self) -> bool {
        (*self.inner).is::<T>()
    }

    pub fn into_inner(self) -> Box<dyn Any> {
        self.inner
    }
}

pub struct AnyMap<K> {
    pub(crate) map: HashMap<K, Value>,
}

impl<K> AnyMap<K> {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        AnyMap {
            map: HashMap::new(),
        }
    }

    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        AnyMap {
            map: HashMap::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.map.capacity()
    }

    #[inline]
    pub fn keys(&self) -> Keys<'_, K, Value> {
        self.map.keys()
    }

    #[inline]
    pub fn values(&self) -> Values<'_, K, Value> {
        self.map.values()
    }

    // TODO: values_mut

    #[inline]
    pub fn iter(&self) -> Iter<'_, K, Value> {
        self.map.iter()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.map.clear()
    }
}

impl<K: Eq + Hash> AnyMap<K> {
    #[inline]
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&Value>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map.get(key)
    }

    #[inline]
    pub fn get_typed<T: Any>(&self, key: &K) -> Option<&'_ T>
    {
        self.map.get(key).and_then(|v| v.as_type::<T>())
    }

    #[inline]
    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map.contains_key(key)
    }

    // TODO: get_mut

    #[inline]
    pub fn insert_val(&mut self, key: K, value: Value) -> Option<Value> {
        self.map.insert(key, value)
    }

    #[inline]
    pub fn insert<T: Any>(&mut self, key: K, value: T) -> Option<Value>
    {
        self.map.insert(key, Value::new(value))
    }

    #[inline]
    pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<Value>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map.remove(key)
    }
}

impl<K> Default for AnyMap<K> {
    #[inline]
    fn default() -> Self {
        AnyMap::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_stores() {
        let val = Value::new(1);
        assert!(val.is::<i32>());
        assert!(!val.is::<String>());
        assert_eq!(val.as_type::<i32>().unwrap(), &1);
    }

    #[test]
    fn value_stores_any() {
        let val = Value::new(1);
        let val2 = Value::new("hello");
        assert!(val.is::<i32>());
        assert!(val2.is::<&str>());
        assert_eq!(val.as_type::<i32>().unwrap(), &1);
        assert_eq!(val2.as_type::<&str>().unwrap(), &"hello");
    }

    #[test]
    fn any_map_stores() {
        let mut map = AnyMap::new();
        map.insert("hello", 1);
        assert_eq!(map.get("hello").unwrap().as_type::<i32>().unwrap(), &1);
        assert!(map.contains_key("hello"));
        assert!(!map.contains_key("world"));
    }

    #[test]
    fn any_map_stores_any() {
        let mut map = AnyMap::new();
        map.insert("hello", 1);
        map.insert("world", "hello");
        assert_eq!(map.get("hello").unwrap().as_type::<i32>().unwrap(), &1);
        assert_eq!(map.get("world").unwrap().as_type::<&str>().unwrap(), &"hello");
        assert!(map.contains_key("hello"));
        assert!(map.contains_key("world"));
    }

    #[test]
    fn value_reports_type() {
        let mut map = AnyMap::new();
        map.insert("key", "value");
        assert!(map.get("key").unwrap().is::<&str>());
    }
}
