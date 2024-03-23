/* 
fn solve_hamilton(matrix: &[Vec<u32>]) -> usize {
    let n = matrix.len();
    // distance is a two dimension map [visited, last_visited]->distance
    let mut distance: HashMap<(usize, usize), usize> = HashMap::new();
    for visited_bin in 0..(1usize << n) {
        for node in 0..n {
            distance.insert((visited_bin, node), usize::max_value());
        }
    }
    distance.insert((1, 0), 0);
    for visited_bin in 2..(1usize << n) {
        for node in 0..n {
            if  (visited_bin >> node) & 1 == 0 {
                continue;
            }
            let mut dist_pre = *distance.get(&(visited_bin, node)).unwrap();
            let visited_before = visited_bin & (!(1 << node));
            for node_before in 0..n {
                if node_before == node || (visited_before >> node_before) & 1 == 0{
                    continue;
                }
                let pre_dist = *distance.get(&(visited_before, node_before)).unwrap();
                if pre_dist < usize::max_value() {
                    let pre_node_to_node_dist: usize =
                        matrix[node_before][node].try_into().unwrap();
                    if pre_dist + pre_node_to_node_dist < dist_pre {
                        dist_pre = pre_dist + pre_node_to_node_dist;
                    }
                }
            }
            distance.insert((visited_bin, node), dist_pre);
        }
    }
    *distance.get(&((1usize << n) - 1, n - 1)).unwrap()
}  */
fn solve_hamilton(matrix: &[Vec<i32>]) -> i32 {
    let n = matrix.len();
    let mut distance = vec![vec![i32::MAX; n]; 1usize << n];
    distance[1][0] = 0;

    for visited_bin in 2..(1 << n) {
        for node in 0..n {
            if (visited_bin >> node) & 1 == 0 {
                continue;
            }
            let mut distance_pre = distance[visited_bin][node];
            let visited_before = visited_bin ^ (1 << node);
            for node_before in 0 ..n {
                if node_before == node || (visited_before >> node_before) & 1 == 0 {
                    continue;
                }
                let pre_dist = distance[visited_before][node_before];
                if pre_dist != i32::MAX {
                    let mut pre_node_to_node_dist = matrix[node_before][node];
                    pre_node_to_node_dist += pre_dist;

                    if  pre_node_to_node_dist < distance_pre {
                        distance_pre = pre_node_to_node_dist;
                    
                    }

                }
            }
            distance[visited_bin][node] = distance_pre;
        }
    }
    distance[(1usize << n) - 1][n - 1]

} 
fn main() {
    let n: u32 = text_io::read!();
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let mut row: Vec<i32> = Vec::new();
        for _ in 0..n {
            let k: i32 = text_io::read!();
            row.push(k);
        }
        matrix.push(row);
    }
    let ans = solve_hamilton(&matrix[..]);
    println!("{ans}");
}
