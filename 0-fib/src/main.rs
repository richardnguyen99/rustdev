use std::io;

fn fib(n: u32) -> u32 {
    if (n == 0) || (n == 1) {
        return n;
    }

    let mut dp: Vec<u32> = vec![0; (n + 1) as usize];

    dp[0] = 0;
    dp[1] = 1;

    for i in 2..=n {
        dp[i as usize] = dp[(i - 1) as usize] + dp[(i - 2) as usize];
    }

    dp[n as usize]
}

fn main() {
    println!("Enter the n-th Fibonacci number you want to calculate: ");

    let mut n: String = String::new();

    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: u32 = n.trim().parse().expect("Please type a number!");

    println!("You entered: {}", n);
    println!("The {}-th Fibonacci number is: {}", n, fib(n));
}
