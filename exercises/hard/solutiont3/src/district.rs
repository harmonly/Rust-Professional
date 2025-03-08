use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn count_provinces() -> String {
    let map = read_and_parse_json(Path::new("./district.json"));

    let mut res_vec = Vec::<i32>::new();

    for i in 1..=map.len() {
        res_vec.push(count(&map[&i.to_string()]));
    }

    res_vec
        .iter()
        .map(|&x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}
fn count(graph: &HashMap<String, Vec<String>>) -> i32 {
    let mut visited = HashMap::<&String, bool>::new();
    let mut res = 0;

    // 复制键到一个新的向量
    let keys: Vec<String> = graph.keys().map(|x| x.to_string()).collect();

    for x in &keys {
        visited.insert(x, false);
    }

    for x in &keys {
        if !visited[&x] {
            res += 1;
            dfs(&x, graph, &mut visited);
        }
    }
    res
}
fn dfs(x: &String, graph: &HashMap<String, Vec<String>>, visited: &mut HashMap<&String, bool>) {
    *visited.get_mut(x).unwrap() = true;

    if let Some(vec) = graph.get(x) {
        for y in vec {
            if !visited[y] {
                dfs(y, graph, visited);
            }
        }
    }
}

fn read_and_parse_json(file_path: &Path) -> HashMap<String, HashMap<String, Vec<String>>> {
    let mut result: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();

    // 尝试打开文件
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("无法打开文件: {}", e);
            return result;
        }
    };
    // 读取文件内容到字符串
    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        eprintln!("无法读取文件内容: {}", e);
        return result;
    }
    let json: Value = match serde_json::from_str(&contents) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("无法解析 JSON 数据: {}", e);
            return result;
        }
    };

    for (top_key, top_value) in json.as_object().unwrap() {
        let mut inner_map: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(cities) = top_value.as_object() {
            for (city, city_list) in cities {
                if let Some(list) = city_list.as_array() {
                    for item in list {
                        if item.as_str().unwrap() == city {
                            continue;
                        }

                        inner_map
                            .entry(item.as_str().unwrap().to_string())
                            .or_insert(Vec::new())
                            .push(city.clone());
                        inner_map
                            .entry(city.to_string())
                            .or_insert(Vec::new())
                            .push(item.as_str().unwrap().to_string());
                    }
                }
            }
        }

        result.insert(top_key.clone(), inner_map);
    }

    result
}
