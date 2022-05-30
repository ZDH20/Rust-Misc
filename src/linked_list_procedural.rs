pub mod linked_list_procedural_mod {
    type NodePtr<T> = Option<Box<Node<T>>>;

    #[derive(Clone)]
    pub struct Node<T> {
        data: T,
        next: NodePtr<T>,
    }

    pub fn node_alloc<T>(d: T) -> NodePtr<T> {
        Some(Box::new(Node { data: d, next: None, }))
    }

    pub fn node_link<T>(passed_node: &mut NodePtr<T>, data: T) {
        assert!(passed_node.is_some());
        let new_node: NodePtr<T> = node_alloc(data);
        // Swap nodes with each other to move `passed_node` to the right spot.
        let tmp_node: NodePtr<T> = std::mem::replace(passed_node, None);
        _                        = std::mem::replace(passed_node, new_node);
        passed_node.as_mut().unwrap().next = tmp_node;
    }

    pub fn node_print_list<T: std::fmt::Debug + Clone>(passed_node: &mut NodePtr<T>) {
        let mut tmp: NodePtr<T> = passed_node.clone();
        while tmp.is_some() {
            println!("{:?}", tmp.as_ref().unwrap().data);
            tmp = tmp.unwrap().next;
        }
    }
}
