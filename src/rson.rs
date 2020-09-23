#![allow(dead_code)]
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::io::{BufReader, Read};

/// JSON Grammar:
///     JSON-text = ws value ws
///
///     ws = *( SPACE | TAB | LINE_FEED/NEW_LINE | CR )
///     
///     Structural Characters:
///         begin-array = ws [ ws
///         begin-object = ws { ws
///         end-array = ws ] ws
///         end-object = ws } ws
///         name-separator = ws : ws
///         value-seperator = ws , ws
///     
///     Values:
///         Value = (false | null | true | object | array | number | string)
///
///     Objects:
///         Object = begin-object [ member *( value-seperator member ) ]
///                  end- object
///
///         member = string name-seperator value
///
///     Arrays:
///         array = begin-array [ value *( value-seperator value ) ] end-array
///
///     Numbers:
///         number = [ minus ] int [ frac ] [ exp ]
///
///         decimal-point = .
///
///         digit1-9 = 1-9
///
///         e = (e | E) ; lowercase | uppercase
///
///         exp = e [ minus | plus ] 1*DIGIT
///
///         frac = decimal-point 1*DIGIT
///
///         int = zero / (digit1-9 *DIGIT)
///
///         minus = -
///
///         plus = +
///
///         zero = 0
///
///     Strings:
///         string = quotation-mark *char quotation-mark
///         char = unescaped | escape ( " | \ | / | b | f | n | r | t | uXXXX )
///         escape = \
///         quotation-mark = "
///         unescaped = a-z | A-Z | %x5D-10FFFF
///
/// From the abover Grammar, we can represent a JSON Value as:
#[derive(Debug, Eq, PartialEq)]
enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(RsonMap<String, Value>),
}

// Constant declarations
const TAB: char = '\t';
const NEW_LINE: char = '\n';
const SPACE: char = ' ';
const BEGIN_OBJECT: char = '{';
const BEGIN_ARRAY: char = '[';
const END_OBJECT: char = '}';
const END_ARRAY: char = ']';

#[derive(Eq, PartialEq)]
#[repr(u8)]
enum StructuralChar {
    BeginArray = '[' as u8,
    EndArray = ']' as u8,
    BeginObject = '{' as u8,
    EndObject = '}' as u8,
    NameSeperator = ':' as u8,
    ValueSeperator = ',' as u8,
    QuotationMark = '"' as u8,
    Unknown,
}

impl StructuralChar {
    fn iter() -> std::slice::Iter<'static, StructuralChar> {
        [
            StructuralChar::BeginArray,
            StructuralChar::BeginObject,
            StructuralChar::EndArray,
            StructuralChar::EndObject,
            StructuralChar::NameSeperator,
            StructuralChar::ValueSeperator,
            StructuralChar::QuotationMark,
        ]
        .iter()
    }
}

impl From<char> for StructuralChar {
    fn from(c: char) -> Self {
        match c {
            '[' => Self::BeginArray,
            ']' => Self::EndArray,
            '{' => Self::BeginObject,
            '}' => Self::EndObject,
            ':' => Self::NameSeperator,
            ',' => Self::ValueSeperator,
            '"' => Self::QuotationMark,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Ops {
    ADD,
    SUB,
    MUL,
    DIV,
    INVALID,
}

impl From<char> for Ops {
    fn from(c: char) -> Self {
        match c {
            '+' => Ops::ADD,
            '-' => Ops::SUB,
            '*' => Ops::MUL,
            '/' => Ops::DIV,
            _ => Ops::INVALID,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct RsonMap<K, V>(HashMap<K, V>)
where
    K: Hash + std::cmp::Ord;

#[derive(Debug, Eq, PartialEq)]
struct Number {
    value: String,
}

impl Number {
    fn new(value: String) -> Self {
        Self { value }
    }
}

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
        [TAB, SPACE].iter().any(|w| Some(*w) == self.look)
    }

    fn match_char(&mut self, x: char) {
        if let Some(look) = self.look {
            if !self.accept(x) {
                panic!("Look: {}, Expected: {}", look, x);
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
            token.push(c);
            self.look = self.get_char();
        }

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
        while let Some(look) = self.look {
            if StructuralChar::from(look) == StructuralChar::EndObject {
                break;
            }
            self.look = self.get_char();
        }
        Value::Null
    }

    fn object(&mut self) -> Value {
        self.skip_white();
        self.match_char('{');
        let mut map = RsonMap(HashMap::new());

        // If we see an END_OBJECT, it's an empty object: {}
        // There is no work to be done here, return early.
        if self.accept(END_OBJECT) {
            return Value::Object(map);
        }

        let key = self.string();
        self.match_char(':');
        let value = self.value();
        self.match_char('}');
        if let Value::String(key) = key {
            map.0.insert(key, value);
        }
        Value::Object(map)
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
        self.match_char('"');

        let mut token = String::new();
        while let Some(c) = self.look {
            if c != '"' {
                token.push(c);
                self.look = self.get_char();
            } else {
                break;
            }
        }

        self.match_char('"');
        self.skip_white();
        Value::String(format!("\"{}\"", token))
    }

    pub fn literal(&mut self) -> Value {
        let token = self.get_token();
        match token.as_str() {
            "null" => Value::Null,
            "true" => Value::Bool(true),
            "false" => Value::Bool(false),
            _ => panic!("Expected a literal"),
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

        Ok(Value::Null)
    }
}

fn expected(value: &str) {
    panic!(format!("Expected a `{}`", value));
}

#[test]
fn test_literal() {
    let mut rson = Rson::from_reader("true".as_bytes()).unwrap();
    assert!(rson.literal() == Value::Bool(true));

    let mut rson = Rson::from_reader("false".as_bytes()).unwrap();
    assert!(rson.literal() == Value::Bool(false));

    let mut rson = Rson::from_reader("null".as_bytes()).unwrap();
    assert!(rson.literal() == Value::Null);
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
fn test_object() {
    let object = r#"{"IsGPU": true}"#;
    let mut rson = Rson::from_reader(object.as_bytes()).unwrap();
    let actual = rson.object();

    let mut map = HashMap::new();
    map.insert(r#""IsGPU""#.to_string(), Value::Null);
    assert_eq!(actual, Value::Object(RsonMap(map)));
}
