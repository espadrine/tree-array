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

// Modified from https://github.com/alexcrichton/splay-rs/blob/master/src/map.rs
//
/// Performs a top-down splay operation on a tree rooted at `node`. This will
/// modify the pointer to contain the new root of the tree once the splay
/// operation is done. When finished, if `index` is in the tree, it will be at the
/// root. Otherwise the closest key to the specified key will be at the root.
fn splay<V>(index: usize, node: &mut Box<Node<V>>) {
    let mut newleft = None;
    let mut newright = None;

    // Eplicitly grab a new scope so the loans on newleft/newright are
    // terminated before we move out of them.
    {
        // Yes, these are backwards, that's intentional.
        let mut l = &mut newright;
        let mut r = &mut newleft;
        let node_idx = node.rel_index();

        loop {
            match index.cmp(&node_idx) {
                // Found it, yay!
                Equal => { break }

                Less => {
                    let mut left = match node.remove_left() {
                        Some(left) => left, None => break
                    };
                    //               left.rel_index() (in LLL… substring)
                    //            |  ↓  |
                    // |----------LLLLLLLNRRRRRR-----|
                    //      left_idx ⬏   ⬑ node_idx
                    let left_idx = node_idx - 1 - match &left.right {
                        Some(right) => right.size, None => 0
                    };
                    // Rotate this node right if necessary.
                    if index.cmp(&left_idx) == Less {
                        // A bit odd, but avoids drop glue
                        mem::swap(&mut node.left, &mut left.right);
                        mem::swap(&mut left, node);
                        let none = mem::replace(&mut node.right, Some(left));
                        match mem::replace(&mut node.left, none) {
                            Some(l) => { left = l; }
                            None    => { break }
                        }
                    }

                    *r = Some(mem::replace(node, left));
                    let tmp = r;
                    r = &mut tmp.as_mut().unwrap().left;
                }

                // If you look closely, you may have seen some similar code
                // before
                Greater => {
                    match node.remove_right() {
                        None => { break }
                        // rotate left if necessary
                        Some(mut right) => {
                            //              right.rel_index() (in RRR… substring)
                            //                    |  ↓ |
                            // |----------LLLLLLLNRRRRRR-----|
                            //          node_idx ⬏   ⬑ right_idx
                            let right_idx = node_idx + 1 + right.rel_index();
                            if index.cmp(&right_idx) == Greater {
                                mem::swap(&mut node.right, &mut right.left);
                                mem::swap(&mut right, node);
                                let none = mem::replace(&mut node.left,
                                                         Some(right));
                                match mem::replace(&mut node.right, none) {
                                    Some(r) => { right = r; }
                                    None    => { break }
                                }
                            }
                            *l = Some(mem::replace(node, right));
                            let tmp = l;
                            l = &mut tmp.as_mut().unwrap().right;
                        }
                    }
                }
            }
        }

        mem::swap(l, &mut node.left);
        mem::swap(r, &mut node.right);
    }

    node.left = newright;
    node.right = newleft;
}
