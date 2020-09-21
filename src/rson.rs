use std::collections::{HashMap, HashSet};
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

struct RsonMap<K, V> {
    map: HashMap<K, V>,
}

struct Number;

struct Rson<'a, R> {
    names: HashSet<&'a str>,
    reader: BufReader<R>,
    look: Option<char>,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl<R: Read> Rson<'_, R> {
    fn from_reader(buf: R) -> Result<Self> {
        let reader = BufReader::new(buf);

        Ok(Self {
            names: HashSet::new(),
            reader,
            look: None,
        })
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
        if self.look != Some(x) {
            panic!("{}", x);
        }
        self.look = self.get_char();
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

    fn object(&mut self) {
        self.skip_white();
        self.match_char('"');
        let name = self.get_name();
        self.match_char('"');
        self.skip_white();
        self.match_char(':');
        self.skip_white();

        print!(" object {}", self.look.unwrap());
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

    fn number(&mut self) {
        print!(" number ");
        self.look = self.get_char();
    }

    fn parse(&mut self) -> Result<Value> {
        self.look = self.get_char();
        while let Some(look) = self.look {
            match look {
                '{' => self.object(),
                '[' => self.array(),
                '}' => self.object(),
                ']' => self.array(),
                _ => {
                    if look.is_ascii_alphabetic() {
                        self.name();
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

#[test]
fn test_rson() {
    let buf = "{ \"key\": \"value\" }";
    let mut rson = Rson::from_reader(buf.as_bytes()).unwrap();
    rson.parse().unwrap();
}
