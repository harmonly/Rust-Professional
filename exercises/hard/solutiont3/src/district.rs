use std::collections::{HashMap, HashSet};
use std::fs;

fn solve(data: &HashMap<String, Vec<String>>) -> usize {
    1
}

pub fn count_provinces() -> String {
    // 读取 JSON 文件
    let data = fs::read_to_string(
        "/home/harmonly/projects/Rust-Professional/exercises/hard/solutiont3/district.json",
    )
    .expect("Unable to read file");

    // 解析 JSON 数据
    let data: HashMap<String, HashMap<String, Vec<String>>> =
        serde_json::from_str(&data).expect("Unable to parse JSON");

    let mut ans: Vec<usize> = Vec::new();
    for i in 0..data.len() {
        ans.push(solve(data.get(&(i + 1).to_string()).unwrap()));
    }
    ans.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
}
