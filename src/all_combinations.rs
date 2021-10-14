#[derive(Clone)]
struct AllCombinationsState {
    finished: bool,
    first: bool,
    current_index: usize,
    current_order: Vec<usize>
}

#[derive(Clone)]
pub struct AllCombinations<T: Clone> {
    list_set: Vec<Vec<T>>,
    state: AllCombinationsState
}

impl<T: Clone> AllCombinations<T> {
    pub fn new(list_set: Vec<Vec<T>>) -> AllCombinations<T> {
        let last_index = list_set.len()-1;
        AllCombinations {
            list_set: list_set.to_vec(),
            state: AllCombinationsState {
                finished: false,
                first: true,
                current_index: last_index,
                current_order: vec![0;list_set.len()]
            }
        }
    }
    fn update(&mut self) -> Option<Option<Vec<T>>> {
        if self.state.finished {
            return Some(None);
        } else {
            let list_set = &self.list_set;
            let last_index = list_set.len()-1;
            let current_index = self.state.current_index;
            if list_set.len() == 1 {
                if current_index >= list_set[0].len() {
                    self.state.finished = true;
                    return Some(None);
                } else {
                    self.state.current_index += 1;
                    return Some(Some(vec![ list_set[0][current_index].clone() ]));
                }
            } else {
                if self.state.first {
                    self.state.first = false;
                    return Some(Some(
                        order_to_values(list_set, &self.state.current_order)
                    ));
                }
                let current_order_index = self.state.current_order[current_index];
                let current_max_order_index = list_set[current_index].len()-1;
                if current_order_index == current_max_order_index {
                    let previous_index = current_index;
                    if self.state.current_index > 0 {
                        self.state.current_index -= 1;
                    }
                    if current_index <= 0 &&
                       self.state.current_order[current_index] ==
                           list_set[current_index].len()-1 {
                        self.state.finished = true;
                        return Some(None);
                    } else {
                        self.state.current_order[previous_index] = 0;
                        return None;
                    }
                } else {
                    self.state.current_order[current_index] += 1;
                    if current_index < last_index {
                        self.state.current_index = last_index;
                    }
                    return Some(Some(
                        order_to_values(list_set, &self.state.current_order)
                    ));
                }
            }
        }
    }
}

fn order_to_values<T: Clone>(list_set: &Vec<Vec<T>>, order: &Vec<usize>) -> Vec<T> {
    return (0..order.len()).zip(
        order.iter()
    ).map(|(index,order)|
          list_set[index as usize][*order as usize].clone())
    .collect();
}

impl<T: Clone> Iterator for AllCombinations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next_value = self.update();
            match next_value {
                Some(value) => return value,
                None => continue
            }
        }
    }
}
