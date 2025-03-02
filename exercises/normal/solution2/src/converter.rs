pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let l = num_str.find("(").unwrap();
    let from_base: u32 = num_str[l + 1..num_str.find(")").unwrap()]
        .to_string()
        .parse()
        .unwrap();

    let mut x = 0;
    for c in num_str[0..l].to_string().chars() {
        let digit = c.to_digit(from_base).unwrap();
        x = x * from_base + digit;
    }

    let mut ans = Vec::new();
    while x > 0 {
        let rem = (x % to_base) as u8;
        let c = match rem {
            0..=9 => ('0' as u8 + rem) as char,
            _ => ('a' as u8 + (rem - 10)) as char,
        };
        ans.push(c);
        x /= to_base;
    }

    ans.iter().rev().collect()
}
