pub fn find_max_prime_factor(mut n: u128) -> u128 {
    let mut ans = 1;

    if n % 2 == 0 {
        ans = 2;
        while n % 2 == 0 {
            n /= 2;
        }
        if n == 1 {
            return ans;
        }
        if miller_rabin(n) {
            return ans.max(n);
        }
    }

    let mut i = 3;
    while i <= n / i {
        if n % i == 0 {
            ans = i;
            while n % i == 0 {
                n /= i;
            }
            if n == 1 {
                return ans;
            }
            if miller_rabin(n) {
                return ans.max(n);
            }
        }
        i += 2;
    }

    if n > 1 {
        ans = ans.max(n);
    }

    ans
}

fn miller_rabin(n: u128) -> bool {
    if n <= 1 {
        return false;
    } else if n <= 3 {
        return true;
    } else if n % 2 == 0 {
        return false;
    }

    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    let bases = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43];
    for &a in &bases {
        if a >= n {
            continue;
        }
        let mut x = q_pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        let mut found = false;
        for _ in 0..s - 1 {
            x = q_pow(x, 2, n);
            if x == n - 1 {
                found = true;
                break;
            }
        }
        if !found {
            return false;
        }
    }
    true
}

fn q_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = b_mul(result, base, modulus);
        }
        base = b_mul(base, base, modulus);
        exp >>= 1;
    }
    result
}

fn b_mul(a: u128, b: u128, modulus: u128) -> u128 {
    let mut a = a % modulus;
    let mut b = b % modulus;
    let mut res = 0;
    while b > 0 {
        if b % 2 == 1 {
            res = (res + a) % modulus;
        }
        a = (a << 1) % modulus;
        b /= 2;
    }
    res
}
