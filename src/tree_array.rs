use std::mem;
use std::cmp::Ordering::{Less, Equal, Greater};

#[derive(Clone)]
pub struct Node<V> {
    pub value: V,
    size: usize,
    left: Option<Box<Node<V>>>,
    right: Option<Box<Node<V>>>,
}

impl<V> Node<V> {
    pub fn new(v: V, l: Option<Box<Node<V>>>, r: Option<Box<Node<V>>>) -> Node<V> {
        Node {
            value: v,
            size: 1,
            left: l,
            right: r,
        }
    }

    // Index of the current node in the slice of the list corresponding to the subtree in which
    // it is root.
    // For instance, in a tree like this:
    //
    //         b
    //        / \
    //       a   x
    //          / \
    //         c   d
    //
    // … node x has relative index 1 (in the slice cxd, which is a part of abcxd).
    #[inline(always)]
    pub fn rel_index(&mut self) -> usize {
        match &self.left {
            None => 0,
            Some(left) => left.size,
        }
    }

    #[inline(always)]
    pub fn remove_left(&mut self) -> Option<Box<Node<V>>> {
        mem::replace(&mut self.left, None)
    }

    #[inline(always)]
    pub fn remove_right(&mut self) -> Option<Box<Node<V>>> {
        mem::replace(&mut self.right, None)
    }
}

pub struct TreeArray<V> {
    root: Option<Box<Node<V>>>
}

// A tree array is a (preferably balanced) binary tree representing a map from indices to
// values, just like an array, where inserting a value increments indices on the right.
// It relies on maintaining the number of nodes in the subtree on each node.
impl<V> TreeArray<V> {
    pub fn new() -> TreeArray<V> {
        TreeArray { root: None }
    }

    // Only works for index 0.
    // TODO: find others.
    pub fn get(&self, index: usize) -> Option<&V> {
        match self.root {
            None => None,
            Some(ref r) => Some(&r.value),
        }
    }

    // Only inserts the first item.
    // TODO: insert anywhere.
    pub fn insert(&mut self, index: usize, value: V) {
        let node = Node::new(value, None, None);
        match self.root {
            Some(ref mut r) => {},
            None => { self.root = Some(Box::new(node)); },
        }
    }

    pub fn len(&self) -> usize {
        match self.root {
            None => 0,
            Some(ref root) => root.size,
        }
    }
}
