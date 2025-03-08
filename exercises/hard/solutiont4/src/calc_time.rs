pub fn time_info(time: &str) -> String {
    let parts: Vec<&str> = time.split('-').collect();
    let y = parts[0].parse::<i32>().unwrap();
    let m = parts[1].parse::<i32>().unwrap();
    let d = parts[2].parse::<i32>().unwrap();

    format!(
        "{},{},{},{},{},{}",
        week_of_year(y, m, d),
        day_of_week(y, m, d),
        day_of_year(y, m, d),
        days_left_in_year(y, m, d),
        days_to_chinese_new_year(y, m, d),
        calc_a_share(y, m, d)
    )
}

// 判断是否为闰年
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

// 获取每个月的天数
fn days_in_month(year: i32, month: i32) -> i32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => panic!("Invalid month"),
    }
}

// 计算当年的第几天
fn day_of_year(year: i32, month: i32, day: i32) -> i32 {
    let mut days = 0;
    for m in 1..month {
        days += days_in_month(year, m);
    }
    days + day
}

// 计算当年剩余天数
fn days_left_in_year(year: i32, month: i32, day: i32) -> i32 {
    (if is_leap_year(year) { 366 } else { 365 }) - day_of_year(year, month, day)
}

// 计算星期几
fn day_of_week(year: i32, month: i32, day: i32) -> i32 {
    let (adjusted_month, adjusted_year) = if month <= 2 {
        (month + 12, year - 1)
    } else {
        (month, year)
    };

    let j = adjusted_year / 100;
    let k = adjusted_year % 100;
    let term2 = (13 * (adjusted_month + 1)) / 5;
    let sum = day + term2 + k + (k / 4) + (j / 4) + 5 * j;
    let q = (sum + 6) % 7;
    if q == 0 {
        7
    } else {
        q
    }
}

// 计算给定日期是该年的第几周
fn week_of_year(year: i32, month: i32, day: i32) -> i32 {
    let days = day_of_year(year, month, day) + 1;
    let total_days = day_of_year(year, 12, 31);
    if days > total_days {
        1
    } else {
        days / 7 + 1
    }
}

fn days_to_chinese_new_year(year: i32, month: i32, day: i32) -> i32 {
    let current_day = day_of_year(year, month, day);
    let new_year_day = day_of_year(year, 1, 29);
    if current_day < new_year_day {
        new_year_day - current_day
    } else {
        let next_year = year + 1;
        let next_new_year_day = day_of_year(next_year, 2, 17);
        let days_in_current_year = if is_leap_year(year) { 366 } else { 365 };
        days_in_current_year - current_day + next_new_year_day
    }
}

fn calc_a_share(year: i32, month: i32, day: i32) -> i32 {
    let mut res: i32 = 0;
    match (month, day) {
        (1, 18) => res = 1,
        (12, 31) => res = 1,
        (11, 1) => res = 1,
        (2, 28) => res = 2,
        (5, 1) => res = 5,
        (1, 28) => res = 7,
        (1, 30) => res = 5,
        _ => (),
    }
    res
}
