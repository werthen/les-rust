extern crate les;
use les::moving;
use les::mutating;

fn main() {
    let mut bst = moving::BST::<i32>::new();
    for i in vec![5, 1, 4, 2] {
        bst = bst.insert(i);
    }
    println!("{:?}", bst);
    println!("{:?}", bst.contains(1));
    println!("{:?}", bst.contains(3));

    let mut mutbst = mutating::BST::<i32>::new();
    for i in vec![5, 1, 4, 2] {
        mutbst.insert(i);
    }
    println!("{:?}", mutbst);
    println!("{:?}", mutbst.contains(1));
    println!("{:?}", mutbst.contains(3));
}
