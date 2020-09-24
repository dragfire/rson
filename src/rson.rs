use crate::value::{Literal, Number, RsonMap, StructuralChar, Value, NEW_LINE, SPACE, TAB};
use std::collections::{HashMap, HashSet};
use std::io::{BufReader, Read};
use std::str::FromStr;

struct Rson<'a, R> {
    names: HashSet<&'a str>,
    reader: BufReader<R>,
    look: Option<char>,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl<R: Read> Rson<'_, R> {
    fn from_reader(buf: R) -> Result<Self> {
        let reader = BufReader::new(buf);

        let mut rson = Self {
            names: HashSet::new(),
            reader,
            look: None,
        };

        rson.look = rson.get_char();

        Ok(rson)
    }

    fn get_char(&mut self) -> Option<char> {
        self.reader
            .by_ref()
            .bytes()
            .next()
            .map(|byte| byte.ok().unwrap() as char)
    }

    /// Skip over leading White Space
    pub fn skip_white(&mut self) {
        while self.is_white() {
            self.look = self.get_char();
        }
    }

    /// Returns true if Lookahead character is TAB or SPACE
    pub fn is_white(&mut self) -> bool {
        [TAB, SPACE, NEW_LINE].iter().any(|w| Some(*w) == self.look)
    }

    fn match_char<T: Into<char>>(&mut self, x: T) {
        if let Some(look) = self.look {
            let c: char = x.into();
            if !self.accept(c) {
                panic!("Look: `{}`, Expected: `{}`", look, c);
            }
        }
        self.look = self.get_char();
        self.skip_white();
    }

    fn accept(&mut self, x: char) -> bool {
        if let Some(look) = self.look {
            if x == look {
                return true;
            }
        }
        false
    }

    fn get_token(&mut self) -> String {
        let mut token = String::new();

        while let Some(c) = self.look {
            if !StructuralChar::iter().any(|&sc| {
                let x: char = sc.into();
                x == c
            }) && !self.is_white()
            {
                token.push(c);
            } else {
                break;
            }
            self.look = self.get_char();
        }

        self.skip_white();

        token
    }

    pub fn get_name(&mut self) -> String {
        let mut token = String::new();
        if let Some(look) = self.look {
            if !look.is_alphabetic() {
                panic!("Name");
            }

            while look.is_ascii_alphanumeric() {
                let look_upcase = look.to_ascii_uppercase();

                token.push(look_upcase);

                self.look = self.get_char();
            }
            self.skip_white();
        }
        token
    }

    pub fn value(&mut self) -> Value {
        // recognize string
        if self.accept(StructuralChar::QuotationMark.into()) {
            return self.string();
        }

        // recognize array
        if self.accept(StructuralChar::BeginArray.into()) {}

        // recognize object
        if self.accept(StructuralChar::BeginObject.into()) {}

        self.literal()
    }

    fn object(&mut self) -> Value {
        self.skip_white();
        self.match_char(StructuralChar::BeginObject);
        let mut map = RsonMap(HashMap::new());

        // If we see an END_OBJECT, it's an empty object: {}
        // There is no work to be done here, return early.
        if self.accept(StructuralChar::EndObject.into()) {
            return Value::Object(map);
        }

        while !self.accept(StructuralChar::EndObject.into()) {
            let key = self.string();
            self.match_char(StructuralChar::NameSeperator);
            let value = self.value();

            // consume ValueSeperator and continue to the next
            // key-value pair if there is any.
            if self.accept(StructuralChar::ValueSeperator.into()) {
                self.match_char(StructuralChar::ValueSeperator);
            }

            if let Value::String(key) = key {
                map.0.insert(key, value);
            }
        }

        self.match_char(StructuralChar::EndObject);
        Value::Object(map)
    }

    fn kv_pair(&mut self) -> Option<(String, Value)> {
        None
    }

    fn array(&mut self) {
        print!(" array ");
        self.look = self.get_char();
    }

    fn name(&mut self) {
        while let Some(c) = self.look {
            print!("{}", c);
            self.look = self.get_char();
        }
    }

    fn string(&mut self) -> Value {
        self.match_char(StructuralChar::QuotationMark);

        let mut token = String::new();
        token.push('"');

        while let Some(c) = self.look {
            if c != StructuralChar::QuotationMark.into() {
                token.push(c);
                self.look = self.get_char();
            } else {
                break;
            }
        }

        self.match_char(StructuralChar::QuotationMark);
        token.push('"');
        self.skip_white();

        Value::String(token)
    }

    pub fn literal(&mut self) -> Value {
        match Literal::from_str(self.get_token().as_str()) {
            Ok(val) => Value::Literal(val),
            Err(e) => panic!(e),
        }
    }

    // TODO support Decimal, Exponent
    fn number(&mut self) -> Value {
        let mut token = String::new();
        if let Some(look) = self.look {
            if !look.is_ascii_digit() {
                expected("Integer");
            }
            while let Some(look) = self.look {
                if look.is_ascii_digit() {
                    token.push(look);
                }
                self.look = self.get_char();
            }
        }
        Value::Number(Number::new(token))
    }

    // TODO revisit
    fn parse(&mut self) -> Result<Value> {
        self.look = self.get_char();
        while let Some(look) = self.look {
            match look {
                _ => {
                    if look.is_ascii_alphabetic() {
                        self.literal();
                    } else if look.is_ascii_digit() {
                        self.number();
                    }
                }
            }
            self.look = self.get_char();
        }

        Ok(Value::Literal(Literal::Null))
    }
}

fn expected(value: &str) {
    panic!(format!("Expected a `{}`", value));
}

#[test]
fn test_literal() {
    let mut rson = Rson::from_reader("true    ".as_bytes()).unwrap();
    assert!(rson.literal() == Value::Literal(Literal::Bool(true)));

    let mut rson = Rson::from_reader("false".as_bytes()).unwrap();
    assert!(rson.literal() == Value::Literal(Literal::Bool(false)));

    let mut rson = Rson::from_reader("null".as_bytes()).unwrap();
    assert!(rson.literal() == Value::Literal(Literal::Null));
}

#[test]
#[should_panic(expected = "Expected a literal")]
fn test_invalid_literal() {
    let mut rson = Rson::from_reader("nullliteral".as_bytes()).unwrap();
    rson.literal();

    let mut rson = Rson::from_reader("true literal".as_bytes()).unwrap();
    rson.literal();

    let mut rson = Rson::from_reader("false literal".as_bytes()).unwrap();
    rson.literal();
}

#[test]
fn test_string() {
    let text = r#""string""#;
    let mut rson = Rson::from_reader(text.as_bytes()).unwrap();
    assert!(rson.string() == Value::String(text.to_string()));

    let text = r#""A long long long string. Maybe!""#;
    let mut rson = Rson::from_reader(text.as_bytes()).unwrap();
    assert!(rson.string() == Value::String(text.to_string()));
}

#[test]
fn test_number() {
    let number = r#"1234213243"#;
    let mut rson = Rson::from_reader(number.as_bytes()).unwrap();
    assert!(rson.number() == Value::Number(Number::new(number.to_string())));
}

#[test]
fn test_object_empty() {
    let object = "{  }";
    let mut rson = Rson::from_reader(object.as_bytes()).unwrap();
    assert!(rson.object() == Value::Object(RsonMap(HashMap::new())));
}

#[test]
#[should_panic]
fn test_object_invalid() {
    let object = "{true}";
    let mut rson = Rson::from_reader(object.as_bytes()).unwrap();
    rson.object();
}

#[test]
fn test_object_string_bool() {
    // { String: Boolean }
    let object = r#"{"IsGPU": true}"#;
    let mut rson = Rson::from_reader(object.as_bytes()).unwrap();
    let actual = rson.object();

    let mut map = HashMap::new();
    map.insert(
        r#""IsGPU""#.to_string(),
        Value::Literal(Literal::Bool(true)),
    );
    assert_eq!(actual, Value::Object(RsonMap(map)));
}

#[test]
fn test_object_string_string() {
    // { String: String }
    let object = r#"{"name": "Devajit Asem"}"#;
    let mut rson = Rson::from_reader(object.as_bytes()).unwrap();
    let actual = rson.object();

    let mut map = HashMap::new();
    map.insert(
        r#""name""#.to_string(),
        Value::String(r#""Devajit Asem""#.to_string()),
    );
    assert_eq!(actual, Value::Object(RsonMap(map)));
}

#[test]
fn test_object_multiple_entries() {
    // {
    //      String: String,
    //      String: Literal
    //      String: Literal
    // }
    //
    let object = r#"{
    "name": "Devajit Asem",
    "hasGPU": true    ,
    "Got3080": null,
    "CanAfford3090"   : false   
    }"#;
    let mut rson = Rson::from_reader(object.as_bytes()).unwrap();
    let actual = rson.object();

    let mut map = HashMap::new();
    map.insert(
        r#""name""#.to_string(),
        Value::String(r#""Devajit Asem""#.to_string()),
    );
    map.insert(
        r#""hasGPU""#.to_string(),
        Value::Literal(Literal::Bool(true)),
    );
    map.insert(r#""Got3080""#.to_string(), Value::Literal(Literal::Null));
    map.insert(
        r#""CanAfford3090""#.to_string(),
        Value::Literal(Literal::Bool(false)),
    );
    assert_eq!(actual, Value::Object(RsonMap(map)));
}
