#[derive(Debug)]
pub struct BST<T> {
    root : Link<T>,
}

impl <T : Ord> BST<T> {
    pub fn new() -> Self {
        BST { root : Link::Empty }
    }

    pub fn insert(&mut self, _val : T) -> bool {
        self.root.insert(_val)
    }

    pub fn search(&self, _val : T) -> bool {
        self.root.search(_val)
    }

    
}
#[derive(Debug)]
enum Link<T> {
    Empty,
    More(Box<Node<T>>),
}

impl<T : Ord> Link<T> {
    pub fn search(&self, _val : T) -> bool {
        match self {
            Link::Empty => {
                false
            }
            Link::More(node) => {
                if _val > node.elem {
                    node.right.search(_val)
                } else if _val < node.elem {
                    node.left.search(_val)
                } else {
                    true
                }
            }
        }
    }

    pub fn insert(&mut self, _val : T) -> bool {
        match self {
            Link::Empty => {
                *self = Link::More(Box::new (Node {elem : _val, left : Link::Empty, right : Link::Empty} ));
                true
            }
            Link::More(node) => {
                if _val > node.elem {
                    node.right.insert(_val)
                } else if _val < node.elem {
                    node.left.insert(_val)
                } else {
                    false
                }
            }
        }
    }
}

#[derive(Debug)]
struct Node<T> {
    elem : T,
    left : Link<T>,
    right : Link<T>,
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
        assert!(my_bst.insert(0));
        assert_eq!(my_bst.search(4), false);
        assert_eq!(my_bst.insert(1), false);
        println!("{:?}", &my_bst);
    }
}