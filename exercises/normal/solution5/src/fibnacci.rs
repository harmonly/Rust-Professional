pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    let mut ans: u32 = 0;
    let (mut a, mut b) = (0, 1);
    let mut fib = a + b;
    while fib < threshold {
        if fib % 2 == 1 {
            ans += fib;
        }
        fib = a + b;
        a = b;
        b = fib;
    }
    ans
}
