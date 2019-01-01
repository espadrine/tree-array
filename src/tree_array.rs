use std::mem;
use std::cmp::Ordering::{Less, Equal, Greater};

#[derive(Clone)]
pub struct Node<V> where V: std::fmt::Display {
    pub value: V,
    size: usize,
    left: Option<Box<Node<V>>>,
    right: Option<Box<Node<V>>>,
}

impl<V> Node<V> where V: std::fmt::Display {
    pub fn new(v: V, l: Option<Box<Node<V>>>, r: Option<Box<Node<V>>>) -> Node<V> {
        let left_size = match &l {
            Some(left) => left.size,
            None => 0,
        };
        let right_size = match &r {
            Some(right) => right.size,
            None => 0,
        };
        let size = 1 + left_size + right_size;
        Node {
            value: v,
            size: size,
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
        let left_size = match &self.left {
            Some(l) => l.size,
            None => 0,
        };
        self.size -= left_size;
        mem::replace(&mut self.left, None)
    }

    #[inline(always)]
    pub fn remove_right(&mut self) -> Option<Box<Node<V>>> {
        let right_size = match &self.right {
            Some(r) => r.size,
            None => 0,
        };
        self.size -= right_size;
        mem::replace(&mut self.right, None)
    }

    pub fn to_str(&self) -> String {
        let left = match &self.left {
            Some(l) => l.to_str(),
            None => String::from("nil"),
        };
        let right = match &self.right {
            Some(r) => r.to_str(),
            None => String::from("nil"),
        };
        std::format!("[{value} size={size}] left=({left}) right=({right})",
            value = self.value, size = self.size, left = left, right = right)
    }
}

pub struct TreeArray<V> where V: std::fmt::Display {
    root: Option<Box<Node<V>>>
}

// A tree array is a (preferably balanced) binary tree representing a map from indices to
// values, just like an array, where inserting a value increments indices on the right.
// It relies on maintaining the number of nodes in the subtree on each node.
impl<V> TreeArray<V> where V: std::fmt::Display {
    pub fn new() -> TreeArray<V> {
        TreeArray { root: None }
    }

    // Only works for index 0.
    pub fn get(&mut self, index: usize) -> Option<&V> {
        match &mut self.root {
            None => None,
            Some(ref mut root) => {
                splay(index, root);
                if index == root.rel_index() {
                    return Some(&root.value);
                }
                None
            }
        }
    }

    // Only inserts the first item.
    pub fn insert(&mut self, index: usize, value: V) {
        match &mut self.root {
            &mut Some(ref mut root) => {
                splay(index, root);
                match index.cmp(&root.rel_index()) {
                    // If equal, the current root will move to the right, and therefore become
                    // of index index+1, which is higher than index.
                    Less | Equal => {
                        let left = root.remove_left();
                        let new = Node::new(value, left, None);
                        let prev = mem::replace(root, Box::new(new));
                        root.size += prev.size;
                        root.right = Some(prev);
                    },
                    Greater => {
                        let right = root.remove_right();
                        let new = Node::new(value, None, right);
                        let prev = mem::replace(root, Box::new(new));
                        root.size += prev.size;
                        root.left = Some(prev);
                    },
                }
            },
            slot => {
                let node = Node::new(value, None, None);
                *slot = Some(Box::new(node));
            }
        }
    }

    pub fn len(&self) -> usize {
        match self.root {
            None => 0,
            Some(ref root) => root.size,
        }
    }

    pub fn to_str(&self) -> String {
        match &self.root {
            None => String::from("nil"),
            Some(r) => r.to_str(),
        }
    }
}

// Modified from https://github.com/alexcrichton/splay-rs/blob/master/src/map.rs
//
/// Performs a top-down splay operation on a tree rooted at `node`. This will
/// modify the pointer to contain the new root of the tree once the splay
/// operation is done. When finished, if `index` is in the tree, it will be at the
/// root. Otherwise the closest key to the specified key will be at the root.
fn splay<V>(index: usize, node: &mut Box<Node<V>>) where V: std::fmt::Display {
    let mut newleft = None;
    let mut newright = None;

    // Explicitly grab a new scope so the loans on newleft/newright are
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
                    //
                    //           L (=left)       N (=node)
                    //  left-left           None   node-right
                    //  (index is in here)
                    if index.cmp(&left_idx) == Less {
                        // A bit odd, but avoids drop glue
                        //          L (=left)           N (=node)
                        // left-left  None    left-right  node-right
                        mem::swap(&mut node.left, &mut left.right);
                        //          L (=node)           N (=left)
                        // left-left  None    left-right  node-right
                        mem::swap(&mut left, node);
                        left.size += match &left.right { Some(lr) => lr.size, None => 0, };
                        node.size = left.size + 1;
                        //           L (=node)
                        // left-left            N (=left)
                        //            left-right  node-right
                        let none = mem::replace(&mut node.right, Some(left));
                        // left-left (=left)         L (=node)
                        //                     None            N
                        //                           left-right  node-right
                        match mem::replace(&mut node.left, none) {
                            Some(l) => { left = l; }
                            None    => { break }
                        }
                    }

                    // L (=node)      N (=tmp)
                    //            None (=r)
                    //
                    // (or)
                    //
                    // left-left (=node)             L (=tmp)
                    //                     None (=r)           N
                    //                               left-right  node-right
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
                                right.size += match &right.left { Some(rl) => rl.size, None => 0, };
                                node.size = right.size + 1;
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
    let mut node_size = 1;
    match &mut node.right {
        Some(ref mut r) => {
            let right_size =
                (match &r.right { None => 0, Some(rr) => rr.size }) +
                (match &r.left  { None => 0, Some(rl) => rl.size });
            r.size = right_size;
            node_size += right_size;
            //println!("after right modification: {}", node.to_str());
        },
        None => {},
    };
    match &mut node.left {
        Some(ref mut l) => {
            let left_size =
                (match &l.right { None => 0, Some(lr) => lr.size }) +
                (match &l.left  { None => 0, Some(ll) => ll.size });
            l.size = left_size;
            node_size += left_size;
            //println!("after left modification: {}", node.to_str());
        },
        None => {},
    };
    node.size = node_size;
}
