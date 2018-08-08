use std::env;

fn main() {
    let arg = env::args().nth(1).unwrap();
    let n: u32 = arg.parse().unwrap();

    let f = fib(n, 0, 1);
    println!("{}", <u128>::max_value());
    println!("{}", f);
    println!("The {} Fibonacci number is: {}", n, f);
}

fn fib(n: u32, a: u128, b: u128) -> u128 {
    if n <= 0 {
        a
    } else {
        fib(n - 1, b, a + b)
    }
}
