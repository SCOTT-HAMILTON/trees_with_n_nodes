use std::cmp;

pub fn repart(r: usize, n: usize, m: Option<usize>) -> Vec<Vec<usize>> {
    let start = match m {
        None => r,
        Some(min) => cmp::min(min,r)
    };
    let mut possibilities = Vec::new();
    for i in (1..start+1).rev() {
        let left = r-i;
        if n-1 < left {
            break
        }
        let mut b = vec![0; n as usize];
        b[(n-1) as usize] = i;
        if left == 0 {
            possibilities.push(b);
        } else if left == 1 {
            b[(n-2) as usize] = 1;
            possibilities.push(b);
        } else if left > 1 {
            let left_possibilities = repart(left, n-1, Some(i));
            for p in left_possibilities {
                let mut concat = p;
                concat.push(i);
                possibilities.push(concat);
            }
        }
    }
    return possibilities;
}
