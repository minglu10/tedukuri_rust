fn solve_hamilton(matrix: &[Vec<u32>]) -> usize {
    let n = matrix.len();
    // distance is a two dimentional map [visited, last_visited]->distance
    let mut distance: Vec<Vec<usize>> = vec![vec![usize::MAX; n]; 1 << n];
    distance[1][0] = 0;
    for visted_bin in 1..(1usize << n) {
        for node in 0..n {
            if (visted_bin >> node) & 1 == 0 {
                continue;
            }
            let mut dist_pre = distance[visted_bin][node];
            let visited_before = visted_bin ^ (1 << node);
            for node_before in 0..n {
                if node_before == node || (visited_before >> node_before) & 1 == 0 {
                    continue;
                }
                let pre_dist = distance[visited_before][node_before];
                if pre_dist < usize::MAX {
                    let mut new_dist: usize = matrix[node_before][node].try_into().unwrap();
                    new_dist += pre_dist;
                    if new_dist < dist_pre {
                        dist_pre = new_dist;
                    }
                }
            }
            distance[visted_bin][node] = dist_pre;
        }
    }
    distance[(1usize << n) - 1][n - 1]
}
fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs::File,
        io::{self, BufRead, BufReader},
    };
    fn read_file_input(file_name: &str) -> io::Result<Vec<Vec<u32>>> {
        let file = File::open(file_name)?;
        let mut reader = BufReader::new(file);
        let mut matrix: Vec<Vec<u32>> = Vec::new();
        let mut first_line = String::new();
        let _ = reader.read_line(&mut first_line);
        let n: usize = first_line.trim().parse::<usize>().unwrap();
        for line in reader.lines() {
            let mut row: Vec<u32> = Vec::new();
            let line_str = line.unwrap();
            let parts = line_str.trim().split(' ');
            for part in parts {
                row.push(part.parse::<u32>().unwrap());
            }
            assert_eq!(n, row.len());
            matrix.push(row);
        }
        Ok(matrix)
    }
    fn read_file_output(file_name: &str) -> io::Result<usize> {
        let file = File::open(file_name)?;
        let mut reader = BufReader::new(file);
        let mut first_line = String::new();
        let _ = reader.read_line(&mut first_line);
        let n = first_line.trim().parse::<usize>().unwrap();
        Ok(n)
    }
    #[test]
    fn tests_hamilton() {
        let files = [
            ("../ham1.in", "../ham1.out"),
            ("../ham2.in", "../ham2.out"),
            ("../ham3.in", "../ham3.out"),
            ("../ham4.in", "../ham4.out"),
            ("../ham5.in", "../ham5.out"),
        ];
        for (input_file, output_file) in files {
            let matrix = read_file_input(input_file).unwrap();
            let output = read_file_output(output_file).unwrap();
            let ans = solve_hamilton(&matrix);
            println!("{} {} {}", input_file, output, ans);
            assert_eq!(output, ans);
        }
    }
}
