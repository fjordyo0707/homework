#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

pub mod first;

#[derive(Debug)]
pub struct BST {
    root : Link,
}

impl BST {
    pub fn new() -> Self {
        BST { root : Link::Empty }
    }

    
}
#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

impl Link {
    pub fn search(&self, _val : i32) -> bool {
        match self {
            Link::Empty => {
                false
            }
            Link::More(node) => {
                match _val.cmp(&node.elem) {
                    std::cmp::Ordering::Less => node.left.search(_val),
                    std::cmp::Ordering::Greater => node.left.search(_val),
                    std::cmp::Ordering::Equal => true,
                }
            }
        }
    }

    pub fn insert(&mut self, _val : i32) -> bool {
        true
    }
}

#[derive(Debug)]
struct Node {
    elem : i32,
    left : Link,
    right : Link,
}