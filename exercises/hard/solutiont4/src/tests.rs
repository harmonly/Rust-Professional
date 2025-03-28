// src/tests.rs
mod calc_time;

#[cfg(test)]
mod tests {
    use super::calc_time::time_info;
    use std::time::{Duration, Instant};

    const TEST_CASES: &[(&str, &str)] = &[
        // 基础测试
        ("2025-01-01", "1,3,1,364,28,0"), // 新年第一天，周三，距春节28天
        ("2025-01-18", "3,6,18,347,11,1"), // 周六，2025年第3周，距春节11天
        // 边界情况测试
        ("2025-12-31", "1,3,365,0,48,1"), // 年末最后一天，周三，距2026年春节48天
        ("2025-11-01", "44,6,305,60,108,1"), // 11月1日，周六，距2026年春节108天
        // 重要日期测试
        ("2025-02-28", "9,5,59,306,354,2"), // 2月的最后一天（非闰年），距2026年春节354天
        ("2025-04-01", "14,2,91,274,322,0"), // 4月1日，周二，距2026年春节322天
        // 春节前后测试
        ("2025-01-28", "5,2,28,337,1,7"), // 春节前一天（周二），距春节1天
        ("2025-01-30", "5,4,30,335,383,5"), // 春节第二天（周四），距2026年春节383天
        // A股开盘日特殊情况
        ("2025-02-09", "6,7,40,325,373,0"), // 周日，距2026年春节373天，A股下个开盘日为2月10日
        ("2025-05-01", "18,4,121,244,292,5"), // 五一劳动节，周四，A股休市，距2026年春节292天
    ];

    // 定义一个测试函数来验证每个测试用例
    #[test]
    fn test_calc_time() {
        let mut total_score = 0.0;
        for (input, expected) in TEST_CASES {
            let start = Instant::now();
            let result = time_info(*input);
            let duration = start.elapsed();

            // 时间超0.2s，判定不合格
            if duration <= Duration::from_millis(200) && result == *expected {
                total_score += 10.0;
            } else {
                println!("{}====={}====={}", input, expected, result);
            }
        }

        println!("Total score: {:.2}", total_score);
        assert_eq!(100.00, total_score);
    }
}
