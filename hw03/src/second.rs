#[derive(Debug)]
pub struct BST<T> {
    root : Link<T>,
}

pub struct IntoIter<T>(BST<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}


impl <T> BST<T> {
    pub fn new() -> Self {
        BST { root : None }
    }

    fn pop_to_right(&mut self) -> Option<T> {
        self.root.take().map(|node| {
            self.root = node.right;
            node.elem
        })
    }
}

impl<T> IntoIterator for BST<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<'a, T> IntoIterator for &'a BST<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Iter<'a, T> {
        Iter{next: self.root.as_ref().map(|node| &**node) }
    }
}

impl<'a, T> IntoIterator for &'a mut BST<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> IterMut<'a, T> {
        IterMut{next: self.root.as_mut().map(|node| &mut **node) }
    }
}

impl <T: Ord> BST<T> {
    pub fn insert(&mut self, _val : T) -> bool {
        self.root.insert(_val)
    }

    pub fn search(&self, _val : T) -> bool {
        self.root.search(_val)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_to_right()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.right.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.right.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}


type Link<T> = Option<Box<Node<T>>>;

trait InsertSearch<T> {
    fn insert(&mut self, e: T) -> bool;
    fn search(&self, e: T) -> bool;
}

impl<T: Ord> InsertSearch<T> for Link<T> {
    fn search(&self, _val : T) -> bool {
        match self {
            None => {
                false
            }
            Some(node) => {
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

    fn insert(&mut self, _val : T) -> bool {
        match self {
            None => {
                *self = Some(Box::new (Node {elem : _val, left : None, right : None} ));
                true
            }
            Link::Some(node) => {
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
        assert!(my_bst.insert(4));
        assert!(my_bst.insert(0));
        assert_eq!(my_bst.search(1), true);
        assert_eq!(my_bst.search(9), false);
        assert_eq!(my_bst.insert(1), false);
        println!("{:?}", &my_bst);
        
        for elt in &my_bst { // calls (&bst).into_iter()
           println!("{}", elt);
        }

        let zero = 0;
        for elt in &mut my_bst { // calls (&mut bst).into_iter()
            println!("{}", elt);
            *elt = zero;
        }

        for elt in my_bst {
            println!("{}", elt);
        }
    }
}