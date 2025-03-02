pub fn goldbach_conjecture() -> String {
    let n = 1e5 as usize;
    let mut prime = vec![true; n];
    for i in 2..n {
        if prime[i] {
            let mut j = i * i;
            while j < n {
                prime[j] = false;
                j += i;
            }
        }
    }
    let mut ans: Vec<usize> = Vec::new();
    for i in 2..n {
        if prime[i] || i % 2 == 0 {
            continue;
        }
        let mut ok = false;
        for j in 2..i {
            if !prime[j] {
                continue;
            }
            if is_perfect_square((i - j) / 2) {
                ok = true;
                break;
            }
        }
        if !ok {
            ans.push(i);
            if ans.len() == 2 {
                break;
            }
        }
    }
    format!("{},{}", ans[0], ans[1])
}

fn is_perfect_square(x: usize) -> bool {
    let p = (x as f64).sqrt().round() as usize;
    p * p == x
}
