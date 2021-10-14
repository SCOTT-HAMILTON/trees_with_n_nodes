use trees::{tr,Tree,Node};

pub fn make_tree(root_childs: usize) -> Tree<String> {
    let mut tree = Tree::<String>::new("R".to_string());
    for i in 0..root_childs {
        tree.push_back( Tree::new((i+1).to_string()) );
    }
    return tree;
}

fn node_to_slab(parent: &mut slab_tree::NodeMut<String>, node: &Node<String>) {
    for child in node.iter() {
        let mut node = parent.append(child.data().to_string());
        node_to_slab(&mut node, child);
    }
}

pub fn tree_str(tree: Tree<String>) -> String {
    let root_children = tree.root().degree();
    let mut stab_tree: slab_tree::Tree<String> = slab_tree::TreeBuilder::new()
        .with_root(tree.root().data().to_string())
        .with_capacity(root_children).build();
    let mut root = stab_tree.root_mut().expect("No root ?");
    node_to_slab(&mut root, tree.root());
    let mut s = String::new();
    stab_tree.write_formatted(&mut s).unwrap();
    return s;
}

pub fn print_tree(tree: &Tree<String>) {
    println!("{}", tree_str(tree.clone()));
}

pub fn trs(data: &str) -> Tree<String> {
    return tr(data.to_string());
}
