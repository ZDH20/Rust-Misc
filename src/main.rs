use rand::Rng;
mod bst {
    type NodeType<T> = Option<Box<Node<T>>>;

    struct Node<T> {
        data:  T,
        left:  NodeType<T>,
        right: NodeType<T>,
    }

    pub struct Tree<T> {
        root: NodeType<T>,
        size: usize,
    }

    impl<T> Node<T> {
        pub fn new(d: T) -> Self {
            Self {
                data: d,
                left: None, right: None,
            }
        }
    }

    impl<T> Tree<T> where
        T: std::fmt::Debug + std::cmp::Ord,
    {
        pub fn new() -> Self { 
            Self {
                root: None,
                size: 0,
            }
        }

        pub fn insert(&mut self, data: T) -> () {
            if self.root.is_none() {
                self.root = Some(Box::new(Node::new(data)));
                self.size += 1;
            } else if Tree::<T>::insert_rec(&mut self.root, data) == true {
                self.size += 1;
            }
        }

        fn insert_rec(cur_node: &mut NodeType<T>, data: T) -> bool {
            if data == cur_node.as_ref().unwrap().data {
                return false;
            }
            if data < cur_node.as_ref().unwrap().data {
                match &cur_node.as_mut().unwrap().left {
                    None => {
                        cur_node.as_mut().unwrap().left = Some(Box::new(Node::new(data)));
                        return true;
                    }
                    Some(_) => return Tree::<T>::insert_rec(&mut cur_node.as_mut().unwrap().left, data),
                }
            } else {
                match &cur_node.as_mut().unwrap().right { 
                    None => {
                        cur_node.as_mut().unwrap().right = Some(Box::new(Node::new(data)));
                        return true;
                    }
                    Some(_) => return Tree::<T>::insert_rec(&mut cur_node.as_mut().unwrap().right, data),
                }
            }
        }

        pub fn contains(&self, data: T) -> bool {
            Tree::<T>::contains_rec(&self.root, data)
        }

        fn contains_rec(cur_node: &NodeType<T>, data: T) -> bool {
            if data < cur_node.as_ref().unwrap().data {
                match &cur_node.as_ref().unwrap().left {
                    None => return false,
                    Some(_) => return Tree::<T>::contains_rec(&cur_node.as_ref().unwrap().left, data),
                }
            } else if data > cur_node.as_ref().unwrap().data {
                match &cur_node.as_ref().unwrap().right { 
                    None => return false,
                    Some(_) => return Tree::<T>::contains_rec(&cur_node.as_ref().unwrap().right, data),
                }
            }
            true
        }

        pub fn size(&self) -> usize {
            self.size
        }
        
        pub fn print(&self) -> () {
            Tree::<T>::print_rec(&self.root);
        }

        fn print_rec(cur_node: &NodeType<T>) -> () {
            if cur_node.is_some() {
                Tree::<T>::print_rec(&cur_node.as_ref().unwrap().right);
                println!("{:?}", cur_node.as_ref().unwrap().data);
                Tree::<T>::print_rec(&cur_node.as_ref().unwrap().left);
            }
        }
    }
}

#[allow(dead_code)]
#[allow(unused_imports)]
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn regular_insert() {
        let mut t = bst::Tree::<i32>::new();
        t.insert(10);
        t.insert(5);
        t.insert(7);
        t.insert(1);
    }

    #[test]
    pub fn contains_test1() {
        let mut t = bst::Tree::<i32>::new();
        t.insert(10);
        t.insert(5);
        t.insert(7);
        t.insert(1);
        assert!(t.contains(10));
        assert!(t.contains(5));
        assert!(t.contains(7));
        assert!(t.contains(1));
        assert!(!t.contains(2));
    }
    
    #[test]
    pub fn big_random_num_test() {
        let mut t = bst::Tree::<i32>::new();
        let mut v = Vec::<i32>::new();
        let mut i: i32;
        for _ in 0..1000000 {
            i = rand::thread_rng().gen_range(0..100000);
            t.insert(i);
            v.push(i);
        }
        for elem in v {
            assert!(t.contains(elem));
        }
    }
    
    #[test]
    pub fn linked_list_test() {
        const SZ: usize = 100;
        let mut t = bst::Tree::<i32>::new();
        for i in 0..SZ {
            t.insert(i as i32);
        }
        for i in 0..SZ {
            assert!(t.contains(i as i32));
        }
    }
}

fn main() {
    let mut t = bst::Tree::<i32>::new();
    for _ in 0..1000000 {
        t.insert(rand::thread_rng().gen_range(0..100000));
    }
    t.print();
}


