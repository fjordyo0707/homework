#[derive(Debug)]
pub struct BST {
    root : Link,
}

impl BST {
    pub fn new() -> Self {
        BST { root : Link::Empty }
    }

    pub fn insert(&mut self, _val : i32) -> bool {
        self.root.insert(_val)
    }

    pub fn search(&self, _val : i32) -> bool {
        self.root.search(_val)
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
                    std::cmp::Ordering::Equal => true,
                    std::cmp::Ordering::Greater => node.right.search(_val),
                    std::cmp::Ordering::Less => node.left.search(_val),
                }
            }
        }
    }

    pub fn insert(&mut self, _val : i32) -> bool {
        match self {
            Link::Empty => {
                *self = Link::More(Box::new (Node {elem : _val, left : Link::Empty, right : Link::Empty} ));
                true
            }
            Link::More(node) => {
                match _val.cmp(&node.elem) {
                    std::cmp::Ordering::Equal => false,
                    std::cmp::Ordering::Greater => {
                        node.right.insert(_val)
                    }
                    std::cmp::Ordering::Less => {
                        node.left.insert(_val)
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
struct Node {
    elem : i32,
    left : Link,
    right : Link,
}

#[cfg(test)]
mod test {
    use super::BST;

    #[test]
    fn test_insert_serch() {
        let mut my_bst = BST::new();
        assert!(my_bst.insert(1));
        assert!(my_bst.insert(2));
        assert!(my_bst.insert(3));
        assert!(my_bst.search(1));
        assert_eq!(my_bst.search(4), false);
        assert_eq!(my_bst.insert(1), false);
        println!("{:?}", &my_bst);
    }
}