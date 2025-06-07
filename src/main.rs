// 実際にコードを書く時は不要なコメントは書かないですが、
// 自分の理解のためにコメント書いてます。
// 型も自分が理解しやすいように明示的に書いています。

use serde_json::{Value as JsonValue, Map, json, to_string_pretty};
use std::collections::HashMap;

fn main() {
    let schema: &'static str = "
endpoint -> string
debug -> bool
log.file -> string
";

    let input: &'static str = "
endpoint = localhost:3000
debug = true
log.file = /var/log/console.log
";

    let schema_map: HashMap<String, String> = parse_schema(schema);
    let mut root: Map<String, JsonValue> = Map::new();


    for line in input.lines() {
        let line: &str = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, val)) = line.split_once('=') {
            let key: &str = key.trim();
            let value_str: &str = val.trim();

            let opt: Option<&String> = schema_map.get(key);
            let expected_type: &String = opt.unwrap_or_else(|| panic!("Unknown key in input: {}", key));

            // matchの比較は&strでないとダメなので変換する
            let value: JsonValue = match expected_type.as_str() {
                "string" => JsonValue::String(value_str.to_string()),
                "bool" => match value_str {
                    "true" => JsonValue::Bool(true),
                    "false" => JsonValue::Bool(false),
                    _ => panic!("Invalid boolean for key '{}': {}", key, value_str),
                },
                _ => panic!("Unsupported type: {}", expected_type),
            };

            let keys: Vec<&str> = key.split('.').collect();
            // rootは可変参照で、keysは不変参照
            insert_nested(&mut root, &keys, value);
        }
    }

    // 型を追いやすいように段階的に書いている
    let json_output: JsonValue = JsonValue::Object(root);
    let json_string_result: Result<String, serde_json::Error> = to_string_pretty(&json_output);
    let json_string: String = json_string_result.unwrap();

    // 1行で書くなら
    // println!("{}", to_string_pretty(&json_output).unwrap());
    println!("{}", json_string);
}

/// スキーマ文字列を key → type の HashMap に変換
fn parse_schema(schema: &str) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();

    for line in schema.lines() {
        let line: &str = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some((key, typ)) = line.split_once("->") {
            map.insert(key.trim().to_string(), typ.trim().to_string());
        }
    }

    map
}

// mapは可変参照, keysは不変参照で受け取る
fn insert_nested(map: &mut Map<String, JsonValue>, keys: &[&str], value: JsonValue) {
    if keys.len() == 1 {
        map.insert(keys[0].to_string(), value);
    } else {
        let key: &str = keys[0];
        // keyが存在しない場合は空のオブジェクトを挿入
        let entry: &mut JsonValue = map.entry(key.to_string()).or_insert_with(|| json!({}));

        // entryが上で挿入したオブジェクトである場合に、入れ子の処理に進む
        // パターンマッチの場合は&mut inner_mapではなくref mut inner_mapと書く
        if let JsonValue::Object(ref mut inner_map) = entry {
            insert_nested(inner_map, &keys[1..], value);
        }
    }
}
