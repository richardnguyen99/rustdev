pub fn fib(n: usize) -> usize {
    if (n == 0) || (n == 1) {
        return n;
    }

    let mut dp: Vec<usize> = vec![0; (n + 1) as usize];

    dp[0] = 0;
    dp[1] = 1;

    for i in 2..=n {
        dp[i as usize] = dp[(i - 1) as usize] + dp[(i - 2) as usize];
    }

    dp[n as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basecase() {
        let result = fib(0);
        assert_eq!(result, 0);

        let result = fib(1);
        assert_eq!(result, 1);
    }

    #[test]
    fn less_than_10() {
        let result = fib(2);
        assert_eq!(result, 1);

        let result = fib(3);
        assert_eq!(result, 2);

        let result = fib(4);
        assert_eq!(result, 3);

        let result = fib(5);
        assert_eq!(result, 5);

        let result = fib(6);
        assert_eq!(result, 8);

        let result = fib(7);
        assert_eq!(result, 13);

        let result = fib(8);
        assert_eq!(result, 21);

        let result = fib(9);
        assert_eq!(result, 34);

        let result = fib(10);
        assert_eq!(result, 55);
    }

    #[test]
    fn wrong_case() {
        let result = fib(11);
        assert_ne!(result, 88);
    }
}
