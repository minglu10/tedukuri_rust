fn flip(values: &mut [[u8; 5]], x: usize, y: usize) {
    values[x][y] ^= 1u8;
    if x > 0 {
        values[x - 1][y] ^= 1u8;
    }
    if x + 1 < 5 {
        values[x + 1][y] ^= 1u8;
    }
    if y > 0 {
        values[x][y - 1] ^=  1u8;
    }
    if y + 1 < 5 {
        values[x][y + 1] ^= 1u8;
    }
}
fn is_all_zero(values: & [[u8; 5]]) ->bool {
    for row in values {
        for  j in row {
            if *j == 0 {
                return false;
            }
        }
    }
    true
}
fn calc (values_org: &[[u8; 5]; 5])->i32 {
    let mut ans = i32::MAX;
    for i in 0..(1u32 << 5) {
        let mut values = *values_org;
        let mut flip_n = 0i32;
        for j in 0..5 {
            if ((1u32 << j) & i) > 0 {
                flip_n += 1;
                flip(&mut values, 0, j);
            }
        }
        for row_n in 1..5 {
            for col_n in 0..5 {
                if values[row_n - 1][col_n] == 0 {
                    flip_n += 1;
                    flip(&mut values, row_n, col_n);
                }
            }
        }
        if is_all_zero(&values) {
            ans = ans.min(flip_n);
        }
    }
    if ans == i32::MAX || ans > 6 {
        ans = -1;
    }
    ans
}
fn main() {
    let n: usize = text_io::read!();
    for _ in 0..n {
        let mut values: [[u8; 5]; 5] = [[0;5];5];
        for i in 0..5 {
            let line:String = text_io::read!();
            for (j, c) in line.chars().enumerate() {
                if c == '1'  {
                    values[i][j] = 1u8;
                }
            }
        }
        println!("{}", calc(&mut values));
    }
}
