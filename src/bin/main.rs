extern crate les;
use les::moving;

fn main() {
    let mut bst = moving::BST::<i32>::new();
    for i in vec![5, 1, 4, 2] {
        bst = bst.insert(i);
    }
    println!("{:?}", bst);
    println!("{:?}", bst.contains(1));
    println!("{:?}", bst.contains(3));
}
