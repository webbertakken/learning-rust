// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables)]

use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

pub fn main() {
    println!("\ncollections...");

    vector();
    hash_map();
    hash_set();
    vector_deque();
    linked_list();
    binary_heap();
    binary_tree_map();
    binary_tree_set();
}

fn vector() {
    // Vector is the most commonly used collection by far.
    // Vec<T>

    // Example
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let x = v.pop(); // x = 3

    println!("{}", v[1]); // prints "2"

    // or using a macro
    let v = vec![1, 2, 3];
}

fn hash_map() {
    // HashMap (in some languages Dictionary)
    let mut h: HashMap<u8, bool> = HashMap::new();
    h.insert(3, true);
    h.insert(4, false);
    let have_three = h.remove(&3).unwrap();

    println!("{}", have_three); // prints "2"
}

fn hash_set() {
    // Hashing implementation of a set
    // Very efficient at set operations
    let hash_set: HashSet<u8> = HashSet::new();
}

fn vector_deque() {
    // Ring buffer with double-ended queue.
    // Fast at adding and removing elements from the start and the end.
    let vector_deque: VecDeque<i32> = VecDeque::new();
}

fn linked_list() {
    // quick at adding or removing items at an arbitrary index.
    // Slow at anything else.
    let linked_list: LinkedList<i32> = LinkedList::new();
}

fn binary_heap() {
    // Priority queue that always pops off the max value
    let binary_heap: BinaryHeap<u8> = BinaryHeap::new();
}

fn binary_tree_map() {
    // Alternate HashMap implementation using a modified binary tree
    // You'd only use this if the hash keys need to always be sorted.
    let binary_tree_map: BTreeMap<u8, bool> = BTreeMap::new();
}

fn binary_tree_set() {
    // Alternate HashSet implementation using a modified binary tree
    // You'd only use this if the hashes need to always be sorted.
    let binary_tree_set: BTreeSet<u8> = BTreeSet::new();
}
