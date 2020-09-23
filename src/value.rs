use std::collections::HashMap;
use std::hash::Hash;

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
pub enum Value {
    Literal(Literal),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(RsonMap<String, Value>),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Literal {
    Null,
    Bool(bool),
}

#[derive(Debug, Eq, PartialEq)]
pub struct RsonMap<K, V>(pub HashMap<K, V>)
where
    K: Hash + std::cmp::Ord;

#[derive(Debug, Eq, PartialEq)]
pub struct Number {
    value: String,
}

impl Number {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

// Constant declarations
pub const TAB: char = '\t';
pub const NEW_LINE: char = '\n';
pub const SPACE: char = ' ';

#[derive(Eq, PartialEq)]
#[repr(u8)]
pub enum StructuralChar {
    BeginArray = '[' as u8,
    EndArray = ']' as u8,
    BeginObject = '{' as u8,
    EndObject = '}' as u8,
    NameSeperator = ':' as u8,
    ValueSeperator = ',' as u8,
    QuotationMark = '"' as u8,
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

impl From<StructuralChar> for char {
    fn from(sc: StructuralChar) -> Self {
        match sc {
            StructuralChar::BeginArray => '[',
            StructuralChar::EndArray => ']',
            StructuralChar::BeginObject => '{',
            StructuralChar::EndObject => '}',
            StructuralChar::NameSeperator => ':',
            StructuralChar::ValueSeperator => ',',
            StructuralChar::QuotationMark => '"',
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
