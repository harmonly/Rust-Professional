pub fn new_birthday_probability(n: u32) -> f64 {
    let mut prob: f64 = 1.0;
    for i in 1..n {
        prob *= (365.0 - i as f64) / 365.0;
    }
    1.0 - prob
}
