#![feature(test)]
extern crate test;

use std::collections::LinkedList;
pub use self::tree_array::TreeArray;

pub mod tree_array;

fn list_insert(l: &mut LinkedList<i32>, position: usize, item: i32) {
    let mut tail = l.split_off(position);
    tail.push_front(item);
    l.append(&mut tail);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_insert_vec(b: &mut Bencher) {
        let mut v = Vec::new();
        for i in 0..1_000_000 {
            v.push(i);
        }
        // Insert a number at the start of the vector, causing maximal harm to speed.
        b.iter(|| v.insert(0, 1));
        println!("v[0] = {}", v[0])
    }

    #[bench]
    fn bench_insert_list(b: &mut Bencher) {
        let mut l = LinkedList::new();
        for i in 0..1_000_000 {
            l.push_back(i);
        }
        // The worst case is insertion at the middle.
        // The start and end are easily accessible in a doubly-linked list.
        b.iter(|| list_insert(&mut l, 500_000, 1));
        println!("l[0] = {}", l.front().unwrap())
    }

    #[test]
    fn test_insert_vec() {
        let mut v = Vec::new();
        v.push(2);
        v.insert(0, 1);
        assert_eq!(v.len(), 2);
        let mut i = 1;
        for e in &v {
            assert_eq!(*e, i);
            i += 1;
        }
    }

    #[test]
    fn test_insert_list() {
        let mut l = LinkedList::new();
        l.push_back(2);
        list_insert(&mut l, 0, 1);
        assert_eq!(l.len(), 2);
        let mut i = 1;
        for e in &l {
            assert_eq!(*e, i);
            i += 1;
        }
    }

    #[test]
    fn test_insert_tree_array() {
        let mut t = TreeArray::new();
        assert_eq!(t.len(), 0);
        t.insert(0, 1);
        t.insert(1, 2);
        println!("tree: {}", t.to_str());
        match t.get(0) {
            None => panic!("Failed to access item at index 0"),
            Some(&v) => assert_eq!(v, 1),
        }
        println!("tree: {}", t.to_str());
        assert_eq!(t.len(), 2);

        //let mut i = 1;
        //for e in &t {
        //    assert_eq!(*e, i);
        //    i += 1;
        //}
    }
}
