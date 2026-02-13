//! Zenith Collections Module
//! Advanced data structures for Zenith standard library

use std::collections::{BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

/// Enhanced vector operations for Zenith
pub struct ZenithVector<T> {
    inner: Vec<T>,
}

impl<T> ZenithVector<T> {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, item: T) {
        self.inner.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.inner.get(index)
    }

    pub fn sort(&mut self)
    where
        T: Ord,
    {
        self.inner.sort();
    }

    pub fn contains(&self, item: &T) -> bool
    where
        T: PartialEq,
    {
        self.inner.contains(item)
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn reverse(&mut self) {
        self.inner.reverse();
    }
}

impl<T> Clone for ZenithVector<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        ZenithVector {
            inner: self.inner.clone(),
        }
    }
}

impl<T> From<Vec<T>> for ZenithVector<T>
where
    T: Clone,
{
    fn from(vec: Vec<T>) -> Self {
        ZenithVector { inner: vec }
    }
}

impl<T> Into<Vec<T>> for ZenithVector<T> {
    fn into(self) -> Vec<T> {
        self.inner
    }
}

/// Enhanced hash map operations for Zenith
pub struct ZenithHashMap<K, V> {
    inner: HashMap<K, V>,
}

impl<K, V> ZenithHashMap<K, V>
where
    K: std::hash::Hash + Eq,
{
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: HashMap::with_capacity(capacity),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.inner.insert(key, value)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.inner.get(key)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.inner.remove(key)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.inner.contains_key(key)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

impl<K, V> Clone for ZenithHashMap<K, V>
where
    K: Clone,
    V: Clone,
{
    fn clone(&self) -> Self {
        ZenithHashMap {
            inner: self.inner.clone(),
        }
    }
}

impl<K, V> From<HashMap<K, V>> for ZenithHashMap<K, V>
where
    K: Clone,
    V: Clone,
{
    fn from(map: HashMap<K, V>) -> Self {
        ZenithHashMap { inner: map }
    }
}

/// Enhanced hash set operations for Zenith
pub struct ZenithHashSet<T> {
    inner: HashSet<T>,
}

impl<T> ZenithHashSet<T>
where
    T: std::hash::Hash + Eq + Clone,
{
    pub fn new() -> Self {
        Self {
            inner: HashSet::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: HashSet::with_capacity(capacity),
        }
    }

    pub fn insert(&mut self, item: T) -> bool {
        self.inner.insert(item)
    }

    pub fn remove(&mut self, item: &T) -> bool {
        self.inner.remove(item)
    }

    pub fn contains(&self, item: &T) -> bool {
        self.inner.contains(item)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

impl<T> Clone for ZenithHashSet<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        ZenithHashSet {
            inner: self.inner.clone(),
        }
    }
}

impl<T> From<HashSet<T>> for ZenithHashSet<T>
where
    T: Clone,
{
    fn from(set: HashSet<T>) -> Self {
        ZenithHashSet { inner: set }
    }
}

/// Enhanced queue operations for Zenith
pub struct ZenithQueue<T> {
    inner: VecDeque<T>,
}

impl<T> ZenithQueue<T> {
    pub fn new() -> Self {
        Self {
            inner: VecDeque::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: VecDeque::with_capacity(capacity),
        }
    }

    pub fn enqueue(&mut self, item: T) {
        self.inner.push_back(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.inner.pop_front()
    }

    pub fn peek(&self) -> Option<&T> {
        self.inner.front()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

impl<T> Clone for ZenithQueue<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        ZenithQueue {
            inner: self.inner.clone(),
        }
    }
}

impl<T> From<VecDeque<T>> for ZenithQueue<T>
where
    T: Clone,
{
    fn from(deque: VecDeque<T>) -> Self {
        ZenithQueue { inner: deque }
    }
}

/// Enhanced stack operations for Zenith
pub struct ZenithStack<T> {
    inner: Vec<T>,
}

impl<T> ZenithStack<T> {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, item: T) {
        self.inner.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.inner.last()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

impl<T> Clone for ZenithStack<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        ZenithStack {
            inner: self.inner.clone(),
        }
    }
}

impl<T> From<Vec<T>> for ZenithStack<T>
where
    T: Clone,
{
    fn from(vec: Vec<T>) -> Self {
        ZenithStack { inner: vec }
    }
}

/// Enhanced linked list operations for Zenith
pub struct ZenithLinkedList<T> {
    inner: LinkedList<T>,
}

impl<T> ZenithLinkedList<T> {
    pub fn new() -> Self {
        Self {
            inner: LinkedList::new(),
        }
    }

    pub fn push_back(&mut self, item: T) {
        self.inner.push_back(item);
    }

    pub fn push_front(&mut self, item: T) {
        self.inner.push_front(item);
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.inner.pop_back()
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.inner.pop_front()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

impl<T> Clone for ZenithLinkedList<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        ZenithLinkedList {
            inner: self.inner.clone(),
        }
    }
}

/// Enhanced priority queue operations for Zenith
pub struct ZenithPriorityQueue<T> {
    inner: BinaryHeap<T>,
}

impl<T> ZenithPriorityQueue<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        Self {
            inner: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.inner.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.inner.peek()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

impl<T> Clone for ZenithPriorityQueue<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        ZenithPriorityQueue {
            inner: self.inner.clone(),
        }
    }
}

/// Collection utilities for Zenith
pub struct CollectionUtils;

impl CollectionUtils {
    /// Create a range vector
    pub fn range(start: i64, end: i64) -> ZenithVector<i64> {
        ZenithVector {
            inner: (start..end).collect(),
        }
    }

    /// Create a vector with repeated values
    pub fn repeat<T>(value: T, count: usize) -> ZenithVector<T>
    where
        T: Clone,
    {
        ZenithVector {
            inner: vec![value; count],
        }
    }

    /// Get minimum element
    pub fn min<T, I>(iter: I) -> Option<T>
    where
        I: Iterator<Item = T>,
        T: Ord,
    {
        iter.min()
    }

    /// Get maximum element
    pub fn max<T, I>(iter: I) -> Option<T>
    where
        I: Iterator<Item = T>,
        T: Ord,
    {
        iter.max()
    }

    /// Get sum of elements
    pub fn sum<T, I>(iter: I) -> T
    where
        I: Iterator<Item = T>,
        T: std::iter::Sum,
    {
        iter.sum()
    }

    /// Get product of elements
    pub fn product<T, I>(iter: I) -> T
    where
        I: Iterator<Item = T>,
        T: std::iter::Product,
    {
        iter.product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zenith_vector() {
        let mut vec = ZenithVector::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);

        assert_eq!(vec.len(), 3);
        assert_eq!(vec.get(0), Some(&1));
        assert_eq!(vec.get(1), Some(&2));
        assert_eq!(vec.get(2), Some(&3));
        assert_eq!(vec.get(3), None);

        assert!(vec.contains(&2));
        assert!(!vec.contains(&4));
    }

    #[test]
    fn test_zenith_hash_map() {
        let mut map = ZenithHashMap::new();
        map.insert("key1".to_string(), "value1".to_string());
        map.insert("key2".to_string(), "value2".to_string());

        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&"key1".to_string()), Some(&"value1".to_string()));
        assert_eq!(map.get(&"key2".to_string()), Some(&"value2".to_string()));
        assert_eq!(map.get(&"key3".to_string()), None);

        assert!(map.contains_key(&"key1".to_string()));
        assert!(!map.contains_key(&"key3".to_string()));
    }

    #[test]
    fn test_zenith_hash_set() {
        let mut set = ZenithHashSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);

        assert_eq!(set.len(), 3);
        assert!(set.contains(&1));
        assert!(set.contains(&2));
        assert!(set.contains(&3));
        assert!(!set.contains(&4));
    }

    #[test]
    fn test_zenith_queue() {
        let mut queue = ZenithQueue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.len(), 3);
        assert_eq!(queue.peek(), Some(&1));

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.len(), 1);
        assert_eq!(queue.peek(), Some(&3));
    }

    #[test]
    fn test_zenith_stack() {
        let mut stack = ZenithStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.len(), 3);
        assert_eq!(stack.peek(), Some(&3));

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.peek(), Some(&1));
    }

    #[test]
    fn test_collection_utils() {
        let vec = ZenithVector::from(vec![1, 2, 3, 4, 5]);

        let min = CollectionUtils::min(vec.inner.iter().cloned());
        assert_eq!(min, Some(1));

        let max = CollectionUtils::max(vec.inner.iter().cloned());
        assert_eq!(max, Some(5));

        let sum = CollectionUtils::sum(vec.inner.iter().cloned());
        assert_eq!(sum, 15);

        let product = CollectionUtils::product(vec.inner.iter().cloned());
        assert_eq!(product, 120);

        let range = CollectionUtils::range(1, 6);
        assert_eq!(range.len(), 5);

        let repeat = CollectionUtils::repeat(42, 3);
        assert_eq!(repeat.len(), 3);
    }
}
