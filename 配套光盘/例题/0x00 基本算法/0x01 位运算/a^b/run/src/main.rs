use text_io::read;
fn power(mut a:u64, mut b:u64, p:u64) -> u64 {
   let mut ans: u64 = 1; 
   while b > 0 {
    if  b & 1 == 1 {
        ans *= a;
        ans %= p;
    }
    a *= a;
    a %= p;
    b >>= 1;
   };
   ans %= p;
   ans
}
fn main() {
    let a: u64 = read!();
    let b: u64 = read!();
    let p: u64 = read!();

    println!("{}", power(a, b, p));
}
