use std::cmp::min;

pub fn retire_time(time: &str, tp: &str) -> String {
    let mut iter = time.split('-');
    let mut year = iter.next().unwrap().parse::<i32>().unwrap();
    let mut month = iter.next().unwrap().parse::<i32>().unwrap();

    let res;
    match tp {
        "男职工" => res = calc(year, month, "man"),
        "原法定退休年龄55周岁女职工" => res = calc(year, month, "woman-55"),
        "原法定退休年龄50周岁女职工" => res = calc(year, month, "woman-50"),
        _ => res = (0, 0.0, 0),
    }
    let (base_retirement_age, retirement_age, delay_months) = res;

    let total_months = retirement_age as i32 * 12 + delay_months;
    month += total_months % 12;
    if month > 12 {
        month -= 12;
        year += 1;
    }
    format!(
        "{}-{:02},{},{}",
        year + total_months / 12,
        month,
        if retirement_age.fract() == 0.0 {
            format!("{:.0}", retirement_age)
        } else {
            format!("{:.2}", retirement_age)
        },
        (retirement_age as i32 - base_retirement_age) * 12 + delay_months
    )
}

fn calc(birth_year: i32, birth_month: i32, tp: &str) -> (i32, f32, i32) {
    let mut base_retirement_age = if tp == "woman-50" {
        50
    } else if tp == "woman-55" {
        55
    } else {
        60
    };

    let mut retirement_age = base_retirement_age as f32;
    let mut delay_months = 0;
    let base_retirement_year = birth_year + base_retirement_age;

    // 政策实施前退休
    if base_retirement_year < 2025 {
        return (base_retirement_age, retirement_age, delay_months);
    }

    // 2025为延迟退休政策起始年
    let total_delay_months_needed = (base_retirement_year - 2025) * 12 + birth_month;

    // 50岁退休女性每年延迟2个月，其他人每年延迟4个月
    let delay_increment = if base_retirement_age == 50 { 2 } else { 4 };
    let actual_delay_months = min(
        // 理论应延迟月数
        (total_delay_months_needed + delay_increment - 1) / delay_increment,
        // 50岁退休最大延迟60个月，其他最大延迟36个月
        if base_retirement_age == 50 { 60 } else { 36 },
    );

    retirement_age += actual_delay_months as f32 / 12.0;
    delay_months = actual_delay_months % 12;
    (base_retirement_age, retirement_age, delay_months)
}
