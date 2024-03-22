use std::collections::HashMap;
fn solve_hamilton(matrix: &[Vec<u32>]) -> usize {
    let n = matrix.len();
    // distance is a two dimentional map [visited, last_visited]->distance
    let mut distance: HashMap<(usize, usize), usize> = HashMap::new();
    for visted_bin in 0..(2usize << n) {
        for node in 0..n {
            distance.insert((visted_bin, node), usize::max_value());
        }
    }
    distance.insert((1, 0), 0);
    for visted_bin in 1..(2usize << n) {
        for node in 0..n {
            let mut dist_pre = *distance.get(&(visted_bin, node)).unwrap();
            let visited_before = visted_bin & (!(1 << node));
            for node_before in 0..n {
                if node_before == node {
                    continue;
                }
                let pre_dist = *distance.get(&(visited_before, node_before)).unwrap();
                if pre_dist < usize::max_value() {
                    let pre_node_to_node_dist: usize =
                        matrix[node_before][node].try_into().unwrap();
                    if pre_dist + pre_node_to_node_dist < dist_pre {
                        dist_pre = pre_dist + pre_node_to_node_dist;
                        distance.insert((visted_bin, node), dist_pre);
                    }
                }
            }
        }
    }
    *distance.get(&((2usize << n) - 1, n - 1)).unwrap()
}
fn main() {
    let n: u32 = text_io::read!();
    let mut matrix: Vec<Vec<u32>> = Vec::new();
    for _ in 0..n {
        let mut row: Vec<u32> = Vec::new();
        for _ in 0..n {
            let k: u32 = text_io::read!();
            row.push(k);
        }
        matrix.push(row);
    }
    let ans = solve_hamilton(&matrix[..]);
    println!("{ans}");
}
