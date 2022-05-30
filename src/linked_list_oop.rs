pub mod linked_list_oop_mod {
    #[derive(Clone)]
    struct Node<T> {
        data: T,
        next: Option<Box<Node<T>>>,
    }

    pub struct List<T> {
        head: Option<Box<Node<T>>>,
    }

    impl<T> Node<T> {
        pub fn new(d: T) -> Self {
            Self {
                data: d,
                next: None,
            }
        }
    }

    impl<T> List<T> where
        T: std::fmt::Debug + Clone,
    {
        pub fn new() -> Self {
            Self {
                head: None,
            }
        }

        pub fn push(&mut self, data: T) -> () {
            let new_node: Node<T> = Node::<T>::new(data);
            match std::mem::replace(&mut self.head, None) {
                None => self.head = Some(Box::new(new_node)),
                Some(cur_node) => {
                    self.head = Some(Box::new(new_node));
                    self.head.as_mut().unwrap().next = Some(cur_node);
                }
            } 
        }

        pub fn dump(&self) -> () {
            let mut tmp = self.head.clone();
            while tmp.is_some() {
                println!("{:?}", tmp.as_ref().unwrap().data);
                tmp = tmp.unwrap().next;
            }
        }
    }
}
