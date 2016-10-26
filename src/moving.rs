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

    pub fn contains(&self, e: T) -> bool {
        e == self.item || if e < self.item {
            self.left.as_ref().map(|node| node.contains(e)).unwrap_or(false)
        } else {
            self.right.as_ref().map(|node| node.contains(e)).unwrap_or(false)
        }
    }

    pub fn insert(self, e: T) -> Self {
        if e <= self.item {
            Node {
                item: self.item,
                left: Some(Box::new(match self.left {
                    None => Node::new(e),
                    Some(node) => node.insert(e)
                })),
                right: self.right
            }
        } else {
            Node {
                item: self.item,
                right: Some(Box::new(match self.right {
                    None => Node::new(e),
                    Some(node) => node.insert(e)
                })),
                left: self.left
            }
        }
    }
}

impl<T: Ord> BST<T> {
    pub fn new() -> Self {
        BST { root: None }
    }

    pub fn insert(self, e: T) -> Self {
        let new_root = match self.root {
            None => Node::new(e),
            Some(node) => node.insert(e)
        };
        BST { root: Some(Box::new(new_root)) }
    }

    pub fn contains(&self, e: T) -> bool {
        self.root.as_ref().map(|node| node.contains(e)).unwrap_or(false)
    }
}
