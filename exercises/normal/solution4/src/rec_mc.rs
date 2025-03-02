pub fn dp_rec_mc(mut amount: u32) -> u32 {
    let arr = vec![1, 2, 5, 10, 20, 30, 50, 100];
    let mut ans: u32 = 0;
    for x in arr.iter().rev() {
        let cnt = amount / x;
        ans += cnt;
        amount -= cnt * x;
    }
    ans
}
