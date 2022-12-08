use std::borrow::{Borrow, BorrowMut};
use std::cmp::Ordering;
use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

#[derive(Debug, Eq, PartialEq)]
enum NodeColor {
    Red,
    Black,
}

/// An implementation of the Node in Red Black Tree
#[derive(Debug, Eq, PartialEq)]
pub struct Node<T> {
    value: T,
    // Option, for the case when the Node is root of the tree
    parent: Option<Rc<RefCell<Node<T>>>>,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
    color: NodeColor,
}

impl<T> Node<T>
where
    T: Ord,
{
    pub fn root(value: T) -> Self {
        Self {
            value,
            parent: None,
            left: None,
            right: None,
            color: NodeColor::Black
        }
    }

    pub fn black(value: T, parent: Rc<RefCell<Node<T>>>) -> Self {
        Self {
            value,
            parent: Some(parent.clone()),
            left: None,
            right: None,
            color: NodeColor::Black
        }
    }

    pub fn red(value: T, parent: Rc<RefCell<Node<T>>>) -> Self {
        Self {
            value,
            parent: Some(parent.clone()),
            left: None,
            right: None,
            color: NodeColor::Red
        }
    }

    fn find(&self, value: &T) -> Option<&Self> {
        if self.value == *value {
            return Some(self);
        }

        if self.value > *value {
            return match self.left {
                Some(ref left) => {
                    let node: RefCell<Node<T>> = *(left.borrow());
                    return (*node).borrow().find(value);
                },
                None => None,
            };
        }

        return match &self.right {
            Some(ref right) => {
                let node: &RefCell<Node<T>> = right.borrow();
                return (*node).borrow().find(value);
            },
            None => None,
        };
    }

    /// The insertion for the Red Black Tree requires two additional processes to be done in
    /// order to provide balanced tree. Additional processes consists of two main actions
    /// 1. Recoloring the nodes of Tree
    /// 1. Rotation the nodes. Rotation means changing relative order and place of some part
    /// of the tree.
    fn insert(&mut self, value: T) {
        fn find_rc<T>(node: &Node<T>) -> Rc<RefCell<Node<T>>> {
            todo!()
        }


        match self.value.cmp(&value) {
            Ordering::Less => match self.right {
                Some(ref mut right) => {
                    let node: &mut RefCell<Node<T>> = right.borrow_mut();
                    (*node).borrow().insert(value)
                },
                _ => {
                    let rc: Rc<RefCell<Node<T>>> = find_rc(&self);
                    let right_node = Some(Rc::new(RefCell::new(Node::red(value, rc))));
                    self.right = right_node;
                },
            },
            Ordering::Greater => match self.left {
                Some(ref mut left) => {
                    let node: &mut RefCell<Node<T>> = left.borrow_mut();
                    (*node).borrow().insert(value)
                },
                _ => {
                    let rc: Rc<RefCell<Node<T>>> = find_rc(&self);
                    let left_node = Some(Rc::new(RefCell::new(Node::red(value, rc))));
                    self.left = left_node;
                },
            },
            Ordering::Equal => {
                // DO NOTHING
            }
        }
    }
}

/// An implementation of classical Red-Black-Tree Data Structure
///
/// The RBTree has it's own rules, to be auto-balanced, and the rules
/// presented in the following points:
/// 1. Every node has a color either red or black.
/// 1. The root of the tree is always black.
/// 1. There are no two adjacent red nodes (A red node cannot have a red parent or red child).
/// 1. Every path from a node (including root) to any of its descendants NULL nodes has the same
/// number of black nodes.
/// 1. All leaf nodes are black nodes.
#[derive(Debug)]
pub struct RedBlackTree<T> {
    root: Option<Rc<RefCell<Node<T>>>>,
    size: usize,
}

impl<T: Ord + Debug> RedBlackTree<T> {
    /// Along with [Default] trait implementation, this method provides a new instance of the Tree.
    /// The consumed value will be placed as a root of the Tree, and the size of this tree will
    /// be set as 1. Hopefully, it's obvious for everyone
    pub fn new(value: T) -> Self {
        let root = Node::root(value);
        Self {
            root: Some(Rc::new(RefCell::new(root))),
            size: 1,
        }
    }

    fn rebalance(&mut self) {
        match self.size {
            0 => {
                // Do nothing, an empty tree is already balanced and has proper colors
            },
            1 => {
                // A Red Black Tree with single node is already balanced, no need to perform any
                // balancing actions
                assert!(self.root.is_some());

                // Double check, that the insertion was correct in terms of color
                let root_node: &RefCell<Node<T>> = self.root.as_ref().unwrap().borrow();
                assert_eq!(root_node.borrow().color, NodeColor::Black);
            },
            2 => {
                // A Red Black Tree with two nodes is already balanced, no need to perform any
                // balancing actions either.
                assert!(self.root.is_some());

                // It's safe to unwrap root option, because the self.size says, that there are
                // two elements in the tree, and we have already checked it above
                // let root = self.root.as_mut().expect("No root in tree with 2 elements");

                // Double check, that root of the tree has Black color
                let root_node: &RefCell<Node<T>> = self.root.as_ref().unwrap().borrow();
                assert_eq!(root_node.borrow().color, NodeColor::Black);

                // Need also double check, that the second node also has a Black color, if not
                // set the color of the child as Black

                let root_node: &RefCell<Node<T>> = self.root.as_ref().unwrap().borrow();
                let root_node: &mut Node<T> = &mut root_node.borrow_mut();

                let mut child: &_ = match (&mut root_node.left, &mut root_node.right) {
                    (Some(ref mut l), None) => (*l).borrow() as &RefCell<Node<T>>,
                    (None, Some(ref mut r)) => (*r).borrow() as &RefCell<Node<T>>,
                    y => {
                        println!("{:?}", y);
                        panic!("Unexpected state of the tree")
                    }
                };
                child.borrow_mut().color = NodeColor::Black;
            },
            _ => {
                unimplemented!()
            }
        }
    }

    /// An insert method adds provided value to the tree. It might perform balancing of the tree
    /// to balance the tree, and keep all algorithmic complexity as O(log(n)). In case, when the
    /// root of the tree is empty ([None]), it will replace root with Node, which is stores
    /// provided value.
    ///
    /// In case, when the provided `value` is already stored in the tree - it will not produce
    /// any changes on the tree, i.e. any data wouldn't be updated.
    ///
    /// An insert method call will increase the size of the tree if the insertion was successful.
    pub fn insert(&mut self, value: T) {
        match self.root {
            Some(ref root) => {
                let node: &RefCell<Node<T>> = root.borrow();
                (*node).borrow().insert(value);
            },
            None => self.root = Some(Rc::new(RefCell::new(Node::root(value)))),
        }

        self.size += 1;
        self.rebalance();
    }

    /// Performs Binary Search on the Tree. It assumes, that the tree is balanced, and properly
    /// ordered. In other cases, the result of this function is not defined. It might, or might
    /// not, give correct result for search request.
    /// In case when the searching value is stored in the tree, it will return reference to
    /// the found node wrapped to Option::[Some]. Otherwise, it will return [None], which is
    /// indicating that the searching value is not exists in the tree.
    pub fn find<K>(&self, value: K) -> Option<&Node<T>>
    where
        K: Borrow<T>,
    {
        self.root.as_ref().map(|r| {
            let node: &RefCell<Node<T>> = r.borrow();
            return (*node).borrow().find(value.borrow());
        })?
    }

    /// Calculates the height of the Tree. In theory the height of the RBTree always
    /// should be as log(n), where the n - is the number of values, stored in the tree
    /// For practicing tree-like algorithms, this method is recursively traversing the
    /// tree. The best implementation might be just calculation of the logarithm of N,
    /// but in this case the developer of this tree must guarantee that the calculation
    /// and actual height of Tree is the same. Otherwise, it might lead for inefficient
    /// behaviours of Tree's methods.
    pub fn height(&self) -> usize {
        fn height<T>(node: &Option<Rc<RefCell<Node<T>>>>) -> usize
        where
            T: Debug,
        {
            match node {
                None => return 0,
                Some(root) => {
                    let node: &RefCell<Node<T>> = root.borrow();
                    let l_h = height(&(*node).borrow().left);
                    let r_h = height(&(*node).borrow().right);

                    1 + std::cmp::max(l_h, r_h)
                },
            }
        }

        let h = height(&self.root);
        let math_h = (self.size as f64).log2().ceil() as usize;
        assert_eq!(h, math_h);
        h
    }
}

impl<T> Default for RedBlackTree<T> {
    fn default() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_tree_will_not_have_any_nodes() {
        let tree: RedBlackTree<i32> = RedBlackTree::default();

        assert_eq!(tree.size, 0);
        assert_eq!(tree.height(), 0);
    }

    #[test]
    fn find_from_empty_tree_will_return_none() {
        let tree: RedBlackTree<i32> = RedBlackTree::default();
        let find_result = tree.find(1);

        assert!(find_result.is_none());
    }

    #[test]
    fn add_to_empty_tree_updates_root() {
        let mut tree: RedBlackTree<i32> = RedBlackTree::default();
        tree.insert(0);

        assert_eq!(tree.size, 1);
        assert!(tree.root.is_some());

        let root = tree.root;

        assert_eq!(root, Some(Rc::new(RefCell::new(Node::root(0)))));
    }

    #[test]
    fn add_to_tree_produces_balanced_tree() {
        let mut tree: RedBlackTree<i32> = RedBlackTree::default();
        tree.insert(2);

        tree.insert(1);
        tree.insert(3);

        assert_eq!(tree.size, 3);
        assert!(tree.root.is_some());
        // TODO: Implement it
        // assert!(tree.root.as_ref().unwrap().right.is_some());
        // assert!(tree.root.as_ref().unwrap().left.is_some());
        println!("{:?}", tree);
    }

    #[test]
    fn find_from_non_empty_tree_must_return_a_value() {
        let mut tree: RedBlackTree<i32> = RedBlackTree::new(2);
        tree.insert(1);
        tree.insert(3);

        assert_eq!(tree.size, 3);

        let root = tree.find(2);
        assert!(root.is_some());
        let root = root.unwrap();
        assert_eq!(root.value, 2);

        let left = tree.find(1);
        assert!(left.is_some());
        let left = left.unwrap();
        assert_eq!(left.value, 1);

        let right = tree.find(3);
        assert!(right.is_some());
        let right = right.unwrap();
        assert_eq!(right.value, 3);

        let not_exist1 = tree.find(10);
        assert!(not_exist1.is_none());

        let not_exist2 = tree.find(-10);
        assert!(not_exist2.is_none());
    }

    #[test]
    fn balanced_tree() {
        let mut tree: RedBlackTree<i32> = RedBlackTree::default();

        for i in 0..5 {
            tree.insert(i);
        }

        // let root_value: Option<i32> = tree.root.as_ref().map(|n| n.value);
        // assert_eq!(root_value, Some(2));

        // let left;
        // let left_left;
        // let left_right;
        // let right_left;
        // let left_right;
        // let leaves;

        assert_eq!(tree.size, 5);
        assert_eq!(tree.height(), 3);
        println!("{:?}", tree);
    }

    #[test]
    fn nested_tree() {
        let mut tree: RedBlackTree<i32> = RedBlackTree::default();

        for i in 0..5 {
            tree.insert(i);
        }

        assert_eq!(tree.size, 5);
        assert_eq!(tree.height(), 3);
        println!("{:?}", tree);
    }
}
