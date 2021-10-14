mod repart;
mod trees_with_n_nodes;
mod apply_repartition;
mod tree_utils;
mod all_combinations;
use trees_with_n_nodes::TreesWithNNodes;
use tree_utils::print_tree;
use argparse::{ArgumentParser,StoreTrue,Store};

fn main() {
    let mut print_trees = false;
    let mut n_arg = "".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Generate all possible trees with n nodes.");
        ap.refer(&mut print_trees)
            .add_option(&["-p", "--print"], StoreTrue,
            "Print all possible trees");
        ap.refer(&mut n_arg)
            .add_argument("N", Store,
                          "number of nodes");
        ap.parse_args_or_exit();
    }

    let n: usize;
    match n_arg.parse::<usize>() {
        Err(why) => {
            println!("Invalid argument for N: `{}`, {}", n_arg, why);
            return;
        },
        Ok(ok_n) => {
            n = ok_n;
        }
    };

    let mut counter = 0;
    for p in TreesWithNNodes::new(n) {
        if print_trees {
            println!("p:");
            print_tree(&p);
        }
        counter += 1;
    }
    println!("There are {} possible trees with {} nodes.", counter, n);
}
