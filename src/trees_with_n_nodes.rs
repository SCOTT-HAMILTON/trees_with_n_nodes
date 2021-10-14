use crate::repart::repart;
use crate::apply_repartition::ApplyRepartition;
use crate::tree_utils::{trs,make_tree};
use trees::Tree;

struct TreesWithNNodesState {
    current_root_childs: usize,
    current_repartitions: Option<Vec<Vec<usize>>>,
    current_repartition_index: usize,
    current_applied_repartitions: Option<ApplyRepartition>
}
pub struct TreesWithNNodes {
    n: usize,
    state: TreesWithNNodesState
}

fn next_repartition_tree(applied_repartitions: &mut ApplyRepartition)
    -> Option<Tree<String>> {
    return applied_repartitions.next();
}
impl TreesWithNNodes {
    pub fn new(n: usize) -> TreesWithNNodes {
        TreesWithNNodes {
            n: n,
            state: TreesWithNNodesState {
                current_root_childs: n-1,
                current_repartitions: None,
                current_repartition_index: 0,
                current_applied_repartitions: None
            }
        }
    }
    fn update(&mut self) -> Option<Option<Tree<String>>> {
        let current_root_childs = self.state.current_root_childs;
        if current_root_childs == 0 {
            return Some(None);
        }
        let mut tree = make_tree(current_root_childs);
        let left = self.n-current_root_childs-1;
        if left == 0 {
            if current_root_childs > 0 {
                self.state.current_root_childs -= 1;
            }
            return Some(Some(tree));
        } else if left == 1 {
            tree.back_mut().unwrap().get_mut().push_back(trs("1"));
            if current_root_childs > 0 {
                self.state.current_root_childs -= 1;
            }
            return Some(Some(tree));
        } else {
            if self.state.current_repartitions == None {
                self.state.current_repartitions = Some(repart(left, current_root_childs, None));
                self.state.current_repartition_index = 0;
                self.state.current_applied_repartitions = Some(
                    ApplyRepartition::new(self.state.current_repartitions.as_ref().unwrap()[0].to_vec())
                );
            }
            if self.state.current_repartition_index >= self.state.current_repartitions.as_ref().unwrap().len() {
                self.state.current_repartitions = None;
                if current_root_childs > 0 {
                    self.state.current_root_childs -= 1;
                }
                return None;
            }
            let next_tree = next_repartition_tree(
                &mut self.state.current_applied_repartitions.as_mut().unwrap()
            );
            if next_tree == None {
                self.state.current_repartition_index += 1;
                if self.state.current_repartition_index >= self.state.current_repartitions.as_ref().unwrap().len() {
                    self.state.current_repartitions = None;
                    if current_root_childs > 0 {
                        self.state.current_root_childs -= 1;
                    }
                    return None;
                }
                self.state.current_applied_repartitions = Some(
                    ApplyRepartition::new(self.state.current_repartitions.as_ref().unwrap().to_vec()[self.state.current_repartition_index].to_vec())
                );
                let next_tree = next_repartition_tree(
                    &mut self.state.current_applied_repartitions.as_mut().unwrap()
                );
                if next_tree == None {
                    return Some(None);
                } else {
                    return Some(Some(next_tree.unwrap()));
                }
            } else {
                return Some(Some(next_tree.unwrap()));
            }
        }
    }
}

impl Iterator for TreesWithNNodes {
    type Item = Tree<String>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let result = self.update();
            if result != None {
                return result.unwrap();
            }
        }
    }
}
