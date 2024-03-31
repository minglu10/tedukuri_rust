use std::io::Cursor;

use text_io;
fn calc(choose: &mut Vec<bool>, current:& mut Vec<usize>, n: usize, idx: usize) {
    if n == idx {
        println!("{:?}", current);
    } else {
        for i in 0..n {
            if choose[i] {
                continue;
            }
            choose[i] = true;
            current[idx] = i + 1;
            calc(choose, current, n, idx + 1);
            choose[i] = false;
        }

    }
}
fn main() {
    let n: usize = text_io::read!();
    let mut choosen: Vec<bool> = vec![false; n];
    let mut current: Vec<usize> = vec![0; n];
    calc(&mut choosen, &mut current, n, 0);
}
