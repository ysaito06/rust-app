use serde_json::{Value as JsonValue, Map, json};

fn main() {
    let input = "
endpoint = localhost:3000
debug = true
log.file = /var/log/console.log
    ";

    let mut root = Map::new();

    for line in input.lines() {
        let line = line.trim();

        // 空行・コメントをスキップ
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // key = value 形式を分解
        if let Some((key, val)) = line.split_once('=') {
            let keys: Vec<&str> = key.trim().split('.').collect();
            let value = JsonValue::String(val.trim().to_string());

            insert_nested(&mut root, &keys, value);
        }
    }

    // 整形された JSON 出力
    let json_output = JsonValue::Object(root);
    println!("{}", serde_json::to_string_pretty(&json_output).unwrap());
}

// ネストされた JSON Map にキーを入れていく
fn insert_nested(map: &mut Map<String, JsonValue>, keys: &[&str], value: JsonValue) {
    if keys.len() == 1 {
        map.insert(keys[0].to_string(), value);
    } else {
        let key = keys[0].to_string();
        let entry = map.entry(key).or_insert_with(|| json!({}));

        if let JsonValue::Object(ref mut inner_map) = entry {
            insert_nested(inner_map, &keys[1..], value);
        }
    }
}
