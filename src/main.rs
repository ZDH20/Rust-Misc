mod tree;
mod linked_list_oop;
mod linked_list_procedural;
use tree::tree_mod;
use linked_list_oop::linked_list_oop_mod;
use linked_list_procedural::linked_list_procedural_mod;

#[allow(unused_variables)]
fn main() {
    let mut binary_search_tree = tree_mod::Tree::<i32>::new();
    let mut oop_list           = linked_list_oop_mod::List::<i32>::new();
    let mut procedural_list    = linked_list_procedural_mod::node_alloc(1); 

    binary_search_tree.insert(1);
    binary_search_tree.print();
  
    oop_list.push(1);
    oop_list.dump();

    linked_list_procedural_mod::node_link(&mut procedural_list, 2);
    linked_list_procedural_mod::node_print_list(&mut procedural_list);
}
