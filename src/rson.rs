use crate::value::{Literal, Number, RsonMap, StructuralChar, Value, NEW_LINE, SPACE, TAB};
use std::collections::{HashMap, HashSet};
use std::io::{BufReader, Read};
use std::str::FromStr;

///
///
/// [(k, v), (k, (k, v))]
macro_rules! map {
    ($e:expr) => {
        let mut map: HashMap<String, Value> = HashMap::new();
    };
}

pub struct Rson<'a, R> {
    names: HashSet<&'a str>,
    reader: BufReader<R>,
    look: Option<char>,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl<R: Read> Rson<'_, R> {
    pub fn from_reader(buf: R) -> Value {
        let reader = BufReader::new(buf);

        let mut rson = Self {
            names: HashSet::new(),
            reader,
            look: None,
        };

        rson.look = rson.get_char();

        rson.parse()
    }

    fn from_str(text: &str) -> Value {
        Rson::from_reader(text.as_bytes())
    }

    fn get_char(&mut self) -> Option<char> {
        self.reader
            .by_ref()
            .bytes()
            .next()
            .map(|byte| byte.ok().unwrap() as char)
    }

    /// Skip over leading White Space
    fn skip_white(&mut self) {
        while self.is_white() {
            self.look = self.get_char();
        }
    }

    /// Returns true if Lookahead character is TAB or SPACE
    fn is_white(&mut self) -> bool {
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

    fn parse(&mut self) -> Value {
        // recognize string
        if self.accept(StructuralChar::QuotationMark.into()) {
            return self.string();
        }

        // recognize array
        if self.accept(StructuralChar::BeginArray.into()) {
            return self.array();
        }

        // recognize object
        if self.accept(StructuralChar::BeginObject.into()) {
            return self.object();
        }

        if let Some(c) = self.look {
            if c.is_ascii_digit() {
                return self.number();
            }
        }

        self.literal()
    }

    fn object(&mut self) -> Value {
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
            let value = self.parse();
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

    fn array(&mut self) -> Value {
        self.match_char(StructuralChar::BeginArray);
        let mut array: Vec<Value> = vec![];
        // If we see an END_OBJECT, it's an empty object: {}
        // There is no work to be done here, return early.
        if self.accept(StructuralChar::EndArray.into()) {
            return Value::Array(array);
        }

        while !self.accept(StructuralChar::EndArray.into()) {
            let value = self.parse();
            // consume ValueSeperator and continue to the next
            // value if there is any.
            if self.accept(StructuralChar::ValueSeperator.into()) {
                self.match_char(StructuralChar::ValueSeperator);
            }
            array.push(value);
        }
        self.match_char(StructuralChar::EndArray);
        Value::Array(array)
    }

    fn string(&mut self) -> Value {
        self.match_char(StructuralChar::QuotationMark);

        let mut token = String::new();

        while let Some(c) = self.look {
            if c != StructuralChar::QuotationMark.into() {
                token.push(c);
                self.look = self.get_char();
            } else {
                break;
            }
        }

        self.match_char(StructuralChar::QuotationMark);
        self.skip_white();

        Value::String(token)
    }

    fn literal(&mut self) -> Value {
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
                } else {
                    break;
                }
                self.look = self.get_char();
            }
        }
        self.skip_white();
        Value::Number(Number::new(token))
    }
}

fn expected(value: &str) {
    panic!(format!("Expected a `{}`", value));
}
