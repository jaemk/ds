/*!
Binary Search Tree

*/
use std::cmp::Ord;


type Link<T> = Option<Box<Node<T>>>;

/// since we can't `impl Link` to make `Link::new`
macro_rules! link {
    ($elem:expr) => {
        Some(Box::new(Node::new($elem)))
    }
}


struct Node<T> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
}
impl<T: Ord> Node<T> {
    fn new(elem: T) -> Self {
        Node {
            elem: elem,
            left: None,
            right: None,
        }
    }

    /// Attempts to insert `elem` into the tree.
    /// Returns `false` if an equal item already exists in the tree, otherwise `true`
    fn insert(&mut self, elem: T) -> bool {
        use std::cmp::Ordering::*;
        match self.elem.cmp(&elem) {
            Equal => return false,
            Greater => {
                // current elem is greater
                if let Some(ref mut left) = self.left {
                    left.insert(elem);
                } else {
                    self.left = link!(elem);
                }
            }
            Less => {
                // current elem is less
                if let Some(ref mut right) = self.right {
                    right.insert(elem);
                } else {
                    self.right = link!(elem);
                }
            }
        }
        true
    }

    /// Walk the tree in search of `elem` while tracking the depth
    fn __find_depth(&self, elem: &T, depth: usize) -> Option<usize> {
        if self.elem == *elem { return Some(depth) }
        if let Some(d) = self.left.as_ref().and_then(|node| node.__find_depth(elem, depth + 1)) {
            return Some(d);
        }
        if let Some(d) = self.right.as_ref().and_then(|node| node.__find_depth(elem, depth + 1)) {
            return Some(d);
        }
        None
    }

    /// Searches for an element and return its depth in the tree
    fn find(&self, elem: &T) -> Option<usize> {
        self.__find_depth(elem, 0)
    }
}


pub struct BTree<T> {
    size: usize,
    head: Link<T>,
}
impl<T: Ord> BTree<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    /// Inserts the given `elem`, incrementing `self.size` if `elem` is
    /// unique and does not already exist in the tree.
    pub fn insert(&mut self, elem: T) {
        let mut inc = false;
        if let Some(ref mut head) = self.head {
            if head.insert(elem) { inc = true }
        } else {
            self.head = link!(elem);
            inc = true;
        }
        if inc { self.size += 1; }
    }

    /// Searches for an element and returns its depth in the tree
    pub fn find(&self, elem: &T) -> Option<usize> {
        if let Some(ref head) = self.head {
            return head.find(elem);
        }
        None
    }

    /// Check if an `elem` is in the tree
    pub fn contains(&self, elem: &T) -> bool {
        self.find(elem).is_some()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_constructs() {
        let bst: BTree<u32> = BTree::new();
        assert_eq!(0, bst.size());
    }

    #[test]
    fn it_inserts() {
        let mut bst = BTree::new();
        bst.insert(50);
        bst.insert(50);
        bst.insert(20);
        bst.insert(60);
        assert_eq!(3, bst.size());
    }

    #[test]
    fn it_finds_depths() {
        let mut bst = BTree::new();
        bst.insert(50);
        bst.insert(50);
        bst.insert(20);
        bst.insert(60);
        assert_eq!(3, bst.size());

        bst.insert(10);
        bst.insert(5);
        bst.insert(70);
        bst.insert(90);

        assert_eq!(Some(0), bst.find(&50));
        assert_eq!(Some(1), bst.find(&20));
        assert_eq!(Some(1), bst.find(&60));
        assert_eq!(Some(2), bst.find(&10));
        assert_eq!(Some(3), bst.find(&5));
        assert_eq!(Some(2), bst.find(&70));
        assert_eq!(Some(3), bst.find(&90));
        assert!(bst.find(&1).is_none());


        let elems: Vec<Vec<String>> = (1..10).map(|n| {
            let v: Vec<String> = (1..n+1).map(|times| {
                let mut s = String::new();
                for _ in 0..times { s.push_str(&format!("{}", n)) }
                s
            }).collect();
            v
        }).collect();
        let mut bst = BTree::new();
        for e in elems.into_iter() {
            bst.insert(e);
        }
        assert_eq!(9, bst.size());
        assert_eq!(Some(0), bst.find(&vec!["1".to_string()]));
    }

    #[test]
    fn it_contains() {
        let mut bst: BTree<String> = BTree::new();
        assert_eq!(false, bst.contains(&format!("anything")));

        bst.insert("something".to_string());
        assert!(bst.contains(&format!("something")));


        let mut bst: BTree<&str> = BTree::new();
        assert_eq!(false, bst.contains(&"anything"));

        bst.insert("something");
        assert!(bst.contains(&"something"));
    }
}
