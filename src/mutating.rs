
type Edge<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct BST<T> {
    pub root: Edge<T>
}

#[derive(Debug)]
pub struct Node<T> {
    item: T,
    left: Edge<T>,
    right: Edge<T>,
}

impl<T: Ord> Node<T> {
    pub fn new(e: T) -> Self {
        Node {
            item: e,
            left: None,
            right: None
        }
    }

    pub fn insert(&mut self, e: T) {
        if e == self.item { return }

        let target = if self.item < e { &mut self.right }
                     else             { &mut self.left };

        match target {
            &mut None => *target = Some(Box::new(Node::new(e))),
            &mut Some(ref mut node) => node.insert(e),
        }
    }

    pub fn contains(&self, e: T) -> bool {
        if self.item == e { return true }

        let target = if self.item < e { &self.right }
                     else             { &self.left };

        match *target {
            None => false,
            Some(ref node) => node.contains(e)
        }
    }
}

impl<T: Ord> BST<T> {
    pub fn new() -> Self {
        BST { root: None }
    }

    pub fn insert(&mut self, e: T) {
        match self.root {
            None => self.root = Some(Box::new(Node::new(e))),
            Some(ref mut node) => node.insert(e),
        }
    }

    pub fn contains(&self, e: T) -> bool {
        match self.root {
            None => false,
            Some(ref node) => node.contains(e),
        }
    }
}
