use std::error::Error;
fn hurt_num(ops:&[(String, u32)], init_input:u32) -> u32 {
    let mut init = init_input;
    for (op, num ) in ops {
        match op.as_str() {
            "AND" => init &= num,
            "OR" =>init |= num, 
            "XOR" =>init ^= num,
            _other => panic!("bad input {op} {num}"),
        }
    }
    init
}
fn max_hurt_to_boss(m:u32, ops: &[(String, u32)]) ->Result<u32, Box<dyn Error>> {
    let mut curr_num:u32 = 0;
    let mut hurt:u32 = 0;
    for i in (0..32u32).rev() {
        if (curr_num | (1u32 << i)) > m {
            let hurt_pos = hurt_num(ops, 0) & (1 << i);
            hurt |= hurt_pos;
        } else {
            let hurt_pos1 = hurt_num(ops, 1 << i) & (1 << i);
            let hurt_pos0 = hurt_num(ops, 0) & ( 1 << i);
            if hurt_pos1 > hurt_pos0 {
                curr_num |= 1 << i;
                hurt |= hurt_pos1
            } else {
                hurt |= hurt_pos0;
            }
        }  
    }
    Ok(hurt)
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs::File,
        io::{self, BufRead, BufReader},
    };
    fn read_file_input(file_name: &str) -> io::Result<(u32, Vec<(String, u32)>)> {
        let file = File::open(file_name)?;
        let mut reader = BufReader::new(file);
        let mut first_line = String::new();
        let _ = reader.read_line(&mut first_line);
        let mut tokens = first_line.split_whitespace();
        let n: u32 = tokens.next().unwrap().parse().unwrap();
        let m: u32 = tokens.next().unwrap().parse().unwrap();
        let mut ops = Vec::new();
        for _i in 0..n {
            let mut line = String::new();
            let _ = reader.read_line(&mut line);
            let mut tokens = line.split_whitespace();
            let op = tokens.next().unwrap().to_string();
            let oprand = tokens.next().unwrap().parse::<u32>().unwrap();
            ops.push((op, oprand));
        }
        Ok((m, ops))
    }
    fn read_file_output(file_name: &str) -> io::Result<u32> {
        let file = File::open(file_name)?;
        let mut reader = BufReader::new(file);
        let mut first_line = String::new();
        let _ = reader.read_line(&mut first_line);
        let n = first_line.trim().parse::<u32>().unwrap();
        Ok(n)
    }
    #[test]
    fn tests_boss_hurts() {
        let files = [
            ("../sleep1.in", "../sleep1.out"),
            ("../sleep2.in", "../sleep2.out"),
            ("../sleep3.in", "../sleep3.out"),
            ("../sleep4.in", "../sleep4.out"),
            ("../sleep5.in", "../sleep5.out"),
            ("../sleep6.in", "../sleep6.out"),
            ("../sleep7.in", "../sleep7.out"),
            ("../sleep8.in", "../sleep8.out"),
            ("../sleep9.in", "../sleep9.out"),
            ("../sleep10.in", "../sleep10.out"),
        ];
        for (input_file, output_file) in files {
            let (m, ops) = read_file_input(input_file).unwrap();
            let output = read_file_output(output_file).unwrap();
            let ans = max_hurt_to_boss(m, &ops).unwrap();
            println!("{} {} {}", input_file, output, ans);
            assert_eq!(output, ans);
        }
    }
}

fn main() {
    println!("Hello, world!");
}
