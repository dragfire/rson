use rson::{self, Literal, Number, Rson, RsonMap, Value};
use std::collections::HashMap;

#[test]
fn test_literal() {
    let actual = Rson::from_reader("true    ".as_bytes());
    assert_eq!(actual, Value::Literal(Literal::Bool(true)));

    let actual = Rson::from_reader("false".as_bytes());
    assert_eq!(actual, Value::Literal(Literal::Bool(false)));

    let actual = Rson::from_reader("null".as_bytes());
    assert_eq!(actual, Value::Literal(Literal::Null));
}

#[test]
#[should_panic(expected = "Expected a literal")]
fn test_invalid_literal() {
    Rson::from_reader("nullliteral".as_bytes());
    Rson::from_reader("true literal".as_bytes());
    Rson::from_reader("false literal".as_bytes());
}

#[test]
fn test_string() {
    let text = r#""string""#;
    let actual = Rson::from_reader(text.as_bytes());
    assert_eq!(actual, Value::String("string".to_string()));

    let text = r#""A long long long string. Maybe!""#;
    let actual = Rson::from_reader(text.as_bytes());
    assert_eq!(
        actual,
        Value::String("A long long long string. Maybe!".to_string())
    );
}

#[test]
fn test_number() {
    let number = r#"1234213243"#;
    let actual = Rson::from_reader(number.as_bytes());
    assert!(actual == Value::Number(Number::new(number.to_string())));
}

#[test]
fn test_object_empty() {
    let object = "{  }";
    let actual = Rson::from_reader(object.as_bytes());
    assert!(actual == Value::Object(RsonMap(HashMap::new())));
}

#[test]
#[should_panic]
fn test_object_invalid() {
    let object = "{true}";
    Rson::from_reader(object.as_bytes());
}

#[test]
fn test_object_string_bool() {
    // { String: Boolean }
    let object = r#"{"IsGPU": true}"#;
    let actual = Rson::from_reader(object.as_bytes());

    let mut map = HashMap::new();
    map.insert(r#"IsGPU"#.to_string(), Value::Literal(Literal::Bool(true)));
    assert_eq!(actual, Value::Object(RsonMap(map)));
}

#[test]
fn test_object_string_string() {
    // { String: String }
    let object = r#"{"name": "Devajit Asem"}"#;
    let actual = Rson::from_reader(object.as_bytes());

    let mut map = HashMap::new();
    map.insert(
        r#"name"#.to_string(),
        Value::String(r#"Devajit Asem"#.to_string()),
    );
    assert_eq!(actual, Value::Object(RsonMap(map)));
}

#[test]
fn test_object_multiple_entries() {
    // {
    //      String: String,
    //      String: Literal
    //      String: Literal
    //      String: Literal
    //      String: Object
    // }
    //
    let object = r#"{
    "Id": 93638382,
    "Name": "Devajit Asem",
    "HasGPU": true    ,
    "Got3080": null,
    "CanAfford3090"   : false,
    "GPUDetail": {
        "RamType": "DDR6",
        "SerialNum": 12837982,
    }
    }"#;
    let actual = Rson::from_reader(object.as_bytes());

    let mut map = HashMap::new();
    map.insert(
        r#"Id"#.to_string(),
        Value::Number(Number::new("93638382".to_string())),
    );
    map.insert(
        r#"Name"#.to_string(),
        Value::String(r#"Devajit Asem"#.to_string()),
    );
    map.insert(r#"HasGPU"#.to_string(), Value::Literal(Literal::Bool(true)));
    map.insert(r#"Got3080"#.to_string(), Value::Literal(Literal::Null));
    map.insert(
        r#"CanAfford3090"#.to_string(),
        Value::Literal(Literal::Bool(false)),
    );

    let mut inner_map = HashMap::new();
    inner_map.insert(
        r#"RamType"#.to_string(),
        Value::String(r#"DDR6"#.to_string()),
    );
    inner_map.insert(
        r#"SerialNum"#.to_string(),
        Value::Number(Number::new("12837982".to_string())),
    );
    map.insert(
        r#"GPUDetail"#.to_string(),
        Value::Object(RsonMap(inner_map)),
    );

    assert_eq!(actual, Value::Object(RsonMap(map)));
}

#[test]
fn test_object_array() {
    // { String: Array }
    let array = r#"{"Name": ["Devajit Asem", 12324, true, false, null]}"#;
    let actual = Rson::from_reader(array.as_bytes());

    let mut map = HashMap::new();
    map.insert(
        r#"Name"#.to_string(),
        Value::Array(vec![
            Value::String(r#"Devajit Asem"#.to_string()),
            Value::Number(Number::new("12324".to_string())),
            Value::Literal(Literal::Bool(true)),
            Value::Literal(Literal::Bool(false)),
            Value::Literal(Literal::Null),
        ]),
    );

    assert_eq!(actual, Value::Object(RsonMap(map)));
}

fn setup_object() -> (&'static str, Value) {
    let object = r#"{
    "Id": 93638382,
    "Name": "Devajit Asem",
    "HasGPU": true    ,
    "Got3080": null,
    "CanAfford3090"   : false,
    "GPUDetail": {
        "RamType": "DDR6",
        "SerialNum": 12837982,
    },
    "Array": ["Devajit Asem", 12324, true, false, null]
    }"#;

    let mut map = HashMap::new();
    map.insert(
        r#"Id"#.to_string(),
        Value::Number(Number::new("93638382".to_string())),
    );
    map.insert(
        r#"Name"#.to_string(),
        Value::String(r#"Devajit Asem"#.to_string()),
    );
    map.insert(r#"HasGPU"#.to_string(), Value::Literal(Literal::Bool(true)));
    map.insert(r#"Got3080"#.to_string(), Value::Literal(Literal::Null));
    map.insert(
        r#"CanAfford3090"#.to_string(),
        Value::Literal(Literal::Bool(false)),
    );

    let mut inner_map = HashMap::new();
    inner_map.insert(
        r#"RamType"#.to_string(),
        Value::String(r#"DDR6"#.to_string()),
    );
    inner_map.insert(
        r#"SerialNum"#.to_string(),
        Value::Number(Number::new("12837982".to_string())),
    );
    map.insert(
        r#"GPUDetail"#.to_string(),
        Value::Object(RsonMap(inner_map)),
    );
    map.insert(
        r#"Array"#.to_string(),
        Value::Array(vec![
            Value::String(r#"Devajit Asem"#.to_string()),
            Value::Number(Number::new("12324".to_string())),
            Value::Literal(Literal::Bool(true)),
            Value::Literal(Literal::Bool(false)),
            Value::Literal(Literal::Null),
        ]),
    );

    (object, Value::Object(RsonMap(map)))
}

#[test]
fn test_parse() {
    let (object, expected) = setup_object();
    let actual = Rson::from_reader(object.as_bytes());
    assert_eq!(actual, expected);
}

#[test]
fn test_object_access_index() {
    let (object_str, _) = setup_object();
    let parsed_object = Rson::from_reader(object_str.as_bytes());

    let mut gpu_detail_map = HashMap::new();
    gpu_detail_map.insert(
        r#"RamType"#.to_string(),
        Value::String(r#"DDR6"#.to_string()),
    );
    gpu_detail_map.insert(
        r#"SerialNum"#.to_string(),
        Value::Number(Number::new("12837982".to_string())),
    );

    assert_eq!(
        parsed_object[r#"HasGPU"#],
        Value::Literal(Literal::Bool(true))
    );
    assert_eq!(
        parsed_object[r#"Name"#],
        Value::String(r#"Devajit Asem"#.to_string()),
    );
    assert_eq!(
        parsed_object[r#"GPUDetail"#],
        Value::Object(RsonMap(gpu_detail_map))
    );
    assert_eq!(
        parsed_object[r#"Array"#],
        Value::Array(vec![
            Value::String(r#"Devajit Asem"#.to_string()),
            Value::Number(Number::new("12324".to_string())),
            Value::Literal(Literal::Bool(true)),
            Value::Literal(Literal::Bool(false)),
            Value::Literal(Literal::Null),
        ]),
    );
}
