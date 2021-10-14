use itertools::Itertools;
use trees::Tree;
use crate::trees_with_n_nodes::TreesWithNNodes;
use crate::tree_utils::{trs,make_tree};
use crate::all_combinations::AllCombinations;

struct ApplyRepartitionState {
    all_combinations: AllCombinations<Vec<Tree<String>>>
}

pub struct ApplyRepartition {
    state: ApplyRepartitionState
}

fn apply_child_possibility(order: Vec<Tree<String>>) -> Tree<String> {
    let mut tree = Tree::<String>::new("R".to_string());
    for ps in order.iter() {
        tree.push_back(ps.clone());
    }
    return tree;
}

impl ApplyRepartition {
    pub fn new(repartitions: Vec<usize>) -> ApplyRepartition {
        let mut groups_possibilities: Vec<Vec<Vec<Tree<String>>>> = vec![];
        for (&r, group) in &repartitions.to_vec().iter().group_by(|&r| r) {
            // println!("key={}, group={:?}",key, );
            let group_size = group.collect::<Vec<&usize>>().len();
            let mut group_c_possibilities: Vec<Tree<String>> = vec![];
            if r == 0 {
                group_c_possibilities.push( make_tree(0) );
            }
            else if r == 1 {
                group_c_possibilities.push( make_tree(1) );
            } else if r > 1 {
                if r == 2 {
                    group_c_possibilities.push(
                        trs("R")
                            /trs("1")
                            /trs("1")
                    );
                    group_c_possibilities.push(
                        trs("R")
                            /(trs("1")/trs("1"))
                    );
                } else {
                    group_c_possibilities.push( make_tree(1) );
                }
                group_c_possibilities = TreesWithNNodes::new(r+1).collect();
            }
            groups_possibilities.push(
                group_c_possibilities.iter()
                    .combinations_with_replacement(group_size)
                    .map(|c| c.iter().map(|&t| t.clone()).collect())
                    .collect()
            );
        }
        ApplyRepartition {
            state: ApplyRepartitionState {
                all_combinations: AllCombinations::new(groups_possibilities)
            }
        }
    }
}

impl Iterator for ApplyRepartition {
    type Item = Tree<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_combination = self.state.all_combinations.next();
        match next_combination {
            None => return None,
            Some(combination) => {
                let childs: Vec<Tree<String>> = combination.iter().flatten()
                    .map(|t|t.clone())
                    .collect();
                return Some(apply_child_possibility(childs));
            }
        }
    }
}
