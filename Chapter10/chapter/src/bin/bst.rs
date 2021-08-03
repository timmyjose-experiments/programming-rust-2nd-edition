use std::fmt::Display;

#[derive(Debug)]
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

impl<T: Ord + Display> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree::Empty
    }

    pub fn insert(&mut self, elem: T) {
        match *self {
            BinaryTree::Empty => *self = BinaryTree::NonEmpty(Box::new(TreeNode::new(elem))),

            BinaryTree::NonEmpty(ref mut node) => {
                if elem <= node.elem {
                    node.left.insert(elem);
                } else {
                    node.right.insert(elem);
                }
            }
        }
    }

    pub fn pre_order(&self) {
        match *self {
            BinaryTree::Empty => {}
            BinaryTree::NonEmpty(ref node) => {
                print!("{} ", node.elem);
                node.left.pre_order();
                node.right.pre_order();
            }
        }
    }

    pub fn in_order(&self) {
        match *self {
            BinaryTree::Empty => {}
            BinaryTree::NonEmpty(ref node) => {
                node.left.in_order();
                print!("{} ", node.elem);
                node.right.in_order();
            }
        }
    }

    pub fn post_order(&self) {
        match *self {
            BinaryTree::Empty => {}
            BinaryTree::NonEmpty(ref node) => {
                node.left.post_order();
                node.right.post_order();
                print!("{} ", node.elem);
            }
        }
    }
}

#[derive(Debug)]
struct TreeNode<T> {
    elem: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

impl<T> TreeNode<T> {
    pub fn new(elem: T) -> Self {
        TreeNode {
            elem: elem,
            left: BinaryTree::Empty,
            right: BinaryTree::Empty,
        }
    }
}

fn main() {
    let mut root = BinaryTree::new();
    root.insert(5);
    root.insert(2);
    root.insert(3);
    root.insert(1);
    root.insert(4);
    root.insert(6);
    root.insert(9);
    root.insert(8);
    root.insert(7);
    root.insert(10);

    root.pre_order();
    println!();

    root.in_order();
    println!();

    root.post_order();
    println!();
}
