mod repart;
mod trees_with_n_nodes;
mod apply_repartition;
mod tree_utils;
mod all_combinations;
use trees_with_n_nodes::TreesWithNNodes;
use tree_utils::print_tree;

fn main() {
    // let child_ps: Vec<Vec<Tree<String>>> = vec![
    //     vec![
    //         make_tree(1)
    //     ],
    //     vec![
    //         make_tree(3),
    //         trs("R")
    //             /trs("1")
    //             /(trs("2")/trs("1")),
    //         trs("R")
    //             /(trs("R")/trs("1")/trs("2")),
    //         trs("R")
    //             /(trs("R")
    //                 /(trs("1")/trs("1"))),
            
    //     ]
    // ];
    // let order_index = [ 0, 3 ];
    // let order: Vec<Tree<String>> = (0..order_index.len()).zip(
    //     order_index.iter()
    // ).map(|(index,order_index)|
    //       child_ps[index as usize][*order_index as usize].clone())
    // .collect();
    // for (index,c) in child_ps.iter().enumerate() {
    //     println!("Possibilities for child#{}", index+1);
    //     for p in c {
    //         println!("P:\n{}", tree_str(p.clone()));
    //     }
    // }
    // let base_tree = make_tree(2);
    // println!("Base tree:");
    // print_tree(&base_tree);

    // let repartition: Vec<usize> = vec![1, 2, 2];
    // for p in ApplyRepartition::new(repartition) {
    //     println!("p:");
    //     print_tree(&p);
    // }
    //
    let mut counter = 0;
    for p in TreesWithNNodes::new(4) {
        println!("p:");
        print_tree(&p);
        counter += 1;
        if counter % 100 == 0 {
            println!("{}", counter);
        }
    }
    println!("{}", counter);
}
