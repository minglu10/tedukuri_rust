fn a_pow_b_mod_m(a:u32, b:u32, m:u32) ->u32 {
    let mut a1 = a % m;
    let mut b1 = b;
    let mut ans = 1u32;
    while b1 > 0 {
        if b1 & 1 > 0 {
            ans = (ans * a1) % m   ;
        }
        b1 >>= 1;
        a1 = (a1 * a1) % m;
    }
    ans % m 
}
#[cfg(test)]
mod tests {
    use std::{fs, io::{self, BufRead, BufReader}};
    use super::*;
    #[test]
    fn test_a_pow_b () {
        assert_eq!(8, a_pow_b_mod_m(2, 3, 10))
    }
    fn read_input(file_name: &str) -> io::Result<Vec<(u32, Vec<(u32, u32)>)>> {
        let file = fs::File::open(file_name)?;
        let mut reader = BufReader::new(file);
        let mut line = String::new();
        let _ = reader.read_line(&mut line);
        let n = line.trim().parse::<u32>().unwrap();
        let mut all_values = Vec::new();
        for _ in 0..n {
            line.clear();
            let _ = reader.read_line(&mut line);
            let m = line.trim().parse::<u32>().unwrap();
            line.clear();
            let _ = reader.read_line(&mut line);
            let k = line.trim().parse::<u32>().unwrap();
            let mut values:Vec<(u32, u32)> = Vec::new();
            for _ in 0..k {
                line.clear();
                let _ = reader.read_line(&mut line);
                let mut token = line.split_whitespace();
                let a = token.next().unwrap().trim().parse::<u32>().unwrap();
                let b = token.next().unwrap().trim().parse::<u32>().unwrap();
                values.push((a, b));
            }
            all_values.push((m, values))
        }
        Ok(all_values)
    }
    fn read_output(file_name: &str) -> io::Result<Vec<u32>> {
        let mut ans = Vec::new();
        let file = fs::File::open(file_name)?;
        let mut reader = BufReader::new(file);
        let mut line = String::new();
        let _ = reader.read_line(&mut line);
        while !line.is_empty() {
            let a = line.trim().parse::<u32>().unwrap();
            ans.push(a);
            line.clear();
            let _ = reader.read_line(&mut line);
        }
        Ok(ans)
    }
    #[test] 
    pub fn test_a_pow_b_all () {
        let input = read_input("../mocneni.in").unwrap();
        let output = read_output("../mocneni.out").unwrap();
        let mut ans = Vec::new();
        for (m, values) in input {
            let mut one_ans = 0;
            for (a, b) in values {
                one_ans = (one_ans + a_pow_b_mod_m(a, b, m)) % m;
            }
            ans.push(one_ans)
        }
        println!("{:?} {:?}", ans, output);
        assert_eq!(ans, output);

    }
}
fn main() {
    println!("Hello, world!");
}
