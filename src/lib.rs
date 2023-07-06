#![allow(dead_code)]

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    mem,
};

const INITIAL_CAPACITY: usize = 16;

pub struct HashTable<K: Eq + Hash + Clone, V: Clone> {
    slots: Vec<Option<(K, V)>>,
    size: usize,
}

impl<K, V> HashTable<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        let slots = vec![None; INITIAL_CAPACITY];

        Self { slots, size: 0 }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if let Some(index) = self.find_slot(&key) {
            self.slots[index] = Some((key, value));
            return;
        }

        if self.size * 2 >= self.slots.len() {
            self.resize();
        }

        let mut index = self.hash(&key);
        let capacity = self.slots.len();

        while let Some((_, _)) = &self.slots[index] {
            index = (index + 1) % capacity;
        }

        self.slots[index] = Some((key, value));
        self.size += 1;
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        if let Some(index) = self.find_slot(key) {
            Some(&self.slots[index].as_ref().unwrap().1)
        } else {
            None
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(index) = self.find_slot(key) {
            let (_, value) = self.slots[index].take().unwrap();
            self.size -= 1;
            Some(value)
        } else {
            None
        }
    }
}

impl<K, V> HashTable<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.slots.len()
    }

    fn find_slot(&self, key: &K) -> Option<usize> {
        let mut index = self.hash(key);
        let capacity = self.slots.len();

        while let Some((ref stored_key, _)) = &self.slots[index] {
            if stored_key == key {
                return Some(index);
            }

            index = (index + 1) % capacity;

            if index == self.hash(key) {
                return None;
            }
        }
        None
    }

    fn resize(&mut self) {
        let new_slots = vec![None; self.slots.len() * 2];
        let old_slots = mem::replace(&mut self.slots, new_slots);
        self.size = 0;

        for slot in old_slots.into_iter().flatten() {
            self.insert(slot.0, slot.1);
        }
    }
}

impl<K, V> Default for HashTable<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut table: HashTable<&str, i32> = HashTable::new();
        table.insert("one", 1);
        table.insert("two", 2);
        table.insert("three", 3);

        assert_eq!(table.get(&"one"), Some(&1));
        assert_eq!(table.get(&"two"), Some(&2));
        assert_eq!(table.get(&"three"), Some(&3));
        assert_eq!(table.get(&"four"), None);
    }

    #[test]
    fn test_insert_and_remove() {
        let mut table: HashTable<&str, i32> = HashTable::new();
        table.insert("one", 1);
        table.insert("two", 2);
        table.insert("three", 3);

        assert_eq!(table.remove(&"two"), Some(2));
        assert_eq!(table.get(&"two"), None);
        assert_eq!(table.size, 2);

        assert_eq!(table.remove(&"one"), Some(1));
        assert_eq!(table.get(&"one"), None);
        assert_eq!(table.size, 1);

        assert_eq!(table.remove(&"four"), None);
    }

    #[test]
    fn test_default() {
        let table: HashTable<&str, i32> = HashTable::default();
        assert_eq!(table.size, 0);
        assert_eq!(table.get(&"key"), None);
    }

    #[test]
    fn test_resize() {
        let mut table: HashTable<i32, i32> = HashTable::new();

        // Insert 32 elements to trigger a resize
        for i in 0..32 {
            table.insert(i, i);
        }

        // Ensure all elements are present
        for i in 0..32 {
            assert_eq!(table.get(&i), Some(&i));
        }

        // Insert 64 more elements to trigger another resize
        for i in 32..96 {
            table.insert(i, i);
        }

        // Ensure all elements are still present
        for i in 0..96 {
            assert_eq!(table.get(&i), Some(&i));
        }

        // Ensure the table size is correct
        assert_eq!(table.size, 96);
        // Ensure the capacity has increased to accommodate the elements
        assert!(table.slots.len() >= 96);
    }
}
