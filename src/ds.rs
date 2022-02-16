use std::cmp::Ordering;
use std::collections::VecDeque;

type Child<T> = Option<Box<Node<T>>>;

pub struct IntoPreOrderIter<T>(T);
pub struct IntoInOrderIter<T>(T);
pub struct IntoPostOrderIter<T>(T);

#[derive(Debug)]
pub struct BST<T: Ord> {
    root: Child<T>,
}

#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: Child<T>,
    right: Child<T>,
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Equal => {}
            Ordering::Less => match self.left {
                None => self.left = Some(Box::from(Node::new(value))),
                Some(ref mut node) => node.insert(value),
            },
            Ordering::Greater => match self.right {
                None => self.right = Some(Box::from(Node::new(value))),
                Some(ref mut node) => node.insert(value),
            },
        }
    }

    fn has_element(&self, value: T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Equal => true,
            Ordering::Less => match self.left {
                None => false,
                Some(ref node) => node.has_element(value),
            },
            Ordering::Greater => match self.right {
                None => false,
                Some(ref node) => node.has_element(value),
            },
        }
    }

    fn retrieve(&self, value: T) -> Option<&T> {
        match value.cmp(&self.value) {
            Ordering::Equal => Some(&self.value),
            Ordering::Less => match self.left {
                None => None,
                Some(ref node) => node.retrieve(value),
            },
            Ordering::Greater => match self.right {
                None => None,
                Some(ref node) => node.retrieve(value),
            },
        }
    }

    fn retrieve_as_mut(&mut self, value: T) -> Option<&mut T> {
        match value.cmp(&self.value) {
            Ordering::Equal => Some(&mut self.value),
            Ordering::Less => match self.left {
                None => None,
                Some(ref mut node) => node.retrieve_as_mut(value),
            },
            Ordering::Greater => match self.right {
                None => None,
                Some(ref mut node) => node.retrieve_as_mut(value),
            },
        }
    }

    fn pre_order_traversal(node: Child<T>, pre_order: &mut VecDeque<T>) {
        if let Some(node) = node {
            pre_order.push_front(node.value);
            Self::pre_order_traversal(node.left, pre_order);
            Self::pre_order_traversal(node.right, pre_order);
        }
    }

    fn in_order_traversal(node: Child<T>, in_order: &mut VecDeque<T>) {
        if let Some(node) = node {
            Self::in_order_traversal(node.left, in_order);
            in_order.push_front(node.value);
            Self::in_order_traversal(node.right, in_order);
        }
    }

    fn post_order_traversal(node: Child<T>, post_order: &mut VecDeque<T>) {
        if let Some(node) = node {
            Self::post_order_traversal(node.left, post_order);
            Self::post_order_traversal(node.right, post_order);
            post_order.push_front(node.value);
        }
    }
}

impl<T: Ord> BST<T> {
    pub fn empty() -> Self {
        Self { root: None }
    }

    pub fn new(value: T) -> Self {
        Self {
            root: Some(Box::from(Node::new(value))),
        }
    }

    pub fn insert(&mut self, value: T) {
        match self.root {
            None => self.root = Some(Box::from(Node::new(value))),
            Some(ref mut node) => node.insert(value),
        }
    }

    pub fn has_element(&self, value: T) -> bool {
        match self.root {
            None => false,
            Some(ref node) => node.has_element(value),
        }
    }

    pub fn retrieve(&self, value: T) -> Option<&T> {
        match self.root {
            None => None,
            Some(ref node) => node.retrieve(value),
        }
    }

    pub fn retrieve_as_mut(&mut self, value: T) -> Option<&mut T> {
        match self.root {
            None => None,
            Some(ref mut node) => node.retrieve_as_mut(value),
        }
    }

    pub fn into_pre_order_iter(self) -> IntoPreOrderIter<VecDeque<T>> {
        let mut pre_order = VecDeque::new();
        Node::pre_order_traversal(self.root, &mut pre_order);
        IntoPreOrderIter(pre_order)
    }

    pub fn into_in_order_iter(self) -> IntoInOrderIter<VecDeque<T>> {
        let mut in_order = VecDeque::new();
        Node::in_order_traversal(self.root, &mut in_order);
        IntoInOrderIter(in_order)
    }

    pub fn into_post_order_iter(self) -> IntoPostOrderIter<VecDeque<T>> {
        let mut post_order = VecDeque::new();
        Node::post_order_traversal(self.root, &mut post_order);
        IntoPostOrderIter(post_order)
    }
}

impl<T: Ord> Iterator for IntoPreOrderIter<VecDeque<T>> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

impl<T: Ord> Iterator for IntoInOrderIter<VecDeque<T>> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

impl<T: Ord> Iterator for IntoPostOrderIter<VecDeque<T>> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

// impl<T: Ord> Drop for BST<T> {
//     fn drop(&mut self) {
//         todo!("I need help with this :sob: :sob:")
//
//         // let mut stack = vec![&self.root];
//         // let mut out: Vec<&LinkNode<T>> = Vec::new();
//         //
//         // while !stack.is_empty() {
//         //     let cur_node = stack.pop().unwrap();
//         //     out.push(cur_node);
//         //
//         //     if let Some(node) = &cur_node.as_ref().unwrap().left {
//         //         stack.push()
//         //     }
//         // }
//     }
// }

#[cfg(test)]
mod bst_test {
    use super::BST;

    #[test]
    fn can_insert_element() {
        let mut bst = BST::new(-1);

        bst.insert(0);
        bst.insert(1);
        bst.insert(1);
        bst.insert(2);

        assert_eq!(bst.retrieve(-1), Some(&-1));
        assert_eq!(bst.retrieve(0), Some(&0));
        assert_eq!(bst.retrieve(1), Some(&1));
        assert_eq!(bst.retrieve(2), Some(&2));
    }

    #[test]
    fn check_element_exists() {
        let mut bst = BST::empty();

        bst.insert(1);
        bst.insert(5);

        assert!(bst.has_element(1));
        assert!(bst.has_element(5));
        assert!(!bst.has_element(10));
    }

    #[test]
    fn retrieve_element_from_bst() {
        let mut bst = BST::empty();
        bst.insert(5);
        bst.insert(10);

        let retrieved_value = bst.retrieve(5);
        let invalid_value = bst.retrieve(15);

        assert_eq!(retrieved_value, Some(&5));
        assert_eq!(invalid_value, None);
    }

    #[test]
    fn retrieve_element_as_mut_and_modify_bst() {
        let mut bst = BST::empty();
        bst.insert(10);
        bst.insert(5);

        let _retrieved_value_as_mut: &mut i32 = bst.retrieve_as_mut(5).unwrap();
        *_retrieved_value_as_mut = 2;

        assert!(bst.has_element(10));
        assert!(bst.has_element(2));
        assert!(!bst.has_element(5));
    }

    #[test]
    fn pre_order_traversal() {
        let mut bst = BST::empty();
        bst.insert(3);
        bst.insert(4);
        bst.insert(5);
        bst.insert(1);
        bst.insert(2);

        let mut pre_order_iter = bst.into_pre_order_iter();

        assert_eq!(pre_order_iter.next(), Some(3));
        assert_eq!(pre_order_iter.next(), Some(1));
        assert_eq!(pre_order_iter.next(), Some(2));
        assert_eq!(pre_order_iter.next(), Some(4));
        assert_eq!(pre_order_iter.next(), Some(5));
        assert_eq!(pre_order_iter.next(), None);
    }

    #[test]
    fn in_order_traversal() {
        let mut bst = BST::empty();
        bst.insert(3);
        bst.insert(4);
        bst.insert(5);
        bst.insert(1);
        bst.insert(2);

        let mut in_order_iter = bst.into_in_order_iter();

        assert_eq!(in_order_iter.next(), Some(1));
        assert_eq!(in_order_iter.next(), Some(2));
        assert_eq!(in_order_iter.next(), Some(3));
        assert_eq!(in_order_iter.next(), Some(4));
        assert_eq!(in_order_iter.next(), Some(5));
        assert_eq!(in_order_iter.next(), None);
    }

    #[test]
    fn post_order_traversal() {
        let mut bst = BST::empty();
        bst.insert(3);
        bst.insert(4);
        bst.insert(5);
        bst.insert(1);
        bst.insert(2);

        let mut post_order_traversal = bst.into_post_order_iter();

        assert_eq!(post_order_traversal.next(), Some(2));
        assert_eq!(post_order_traversal.next(), Some(1));
        assert_eq!(post_order_traversal.next(), Some(5));
        assert_eq!(post_order_traversal.next(), Some(4));
        assert_eq!(post_order_traversal.next(), Some(3));
        assert_eq!(post_order_traversal.next(), None);
    }
}