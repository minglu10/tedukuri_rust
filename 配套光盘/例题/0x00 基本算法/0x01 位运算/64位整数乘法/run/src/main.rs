use text_io::read;
fn multiply(mut a:u64, mut b:u64, p: u64) ->u64 {
    let mut ans: u64 = 0;
    while  b > 0 {
        if b & 1 == 1 {
            ans += a;
            ans %= p;
        }
        a *= 2;
        a %= p;
        b >>= 1;
    }
    ans %= p;
    ans
}
fn main() {
    let a: u64 = read!();
    let b: u64 = read!();
    let p: u64 = read!();
    let ans = multiply(a, b, p);
    println!("{ans}");

}
