fn main() {
    let x = 1;
    for i in 1..=10000000 {
        x *= i;
        x %= 1000000007;
    }
    println!("Hello, world! {}", x);
}