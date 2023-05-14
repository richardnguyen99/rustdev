use fibonacci::fib;
use std::io;

fn main() {
    println!("Enter the n-th Fibonacci number you want to calculate: ");

    let mut n: String = String::new();

    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: u32 = n.trim().parse().expect("Please type a number!");

    println!("You entered: {}", n);
    println!("The {}-th Fibonacci number is: {}", n, fib(n as usize));
}
