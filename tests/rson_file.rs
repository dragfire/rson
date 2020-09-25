use rson::{self, Literal, Number, Rson, RsonMap, Value};
use std::fs::File;

fn open_file(filename: &str) -> File {
    let mut path = std::env::current_dir().unwrap();
    path.push("data/");
    path.push(filename);

    File::open(path).unwrap()
}

#[test]
fn test_open_file() {
    let json = Rson::from_reader(open_file("test.json"));
    assert_eq!(json, Value::Literal(Literal::Null));
}
