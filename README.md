# rson
A minimal JSON parser in Rust
        
## Usage:
Right now, `rson` doesn't do anything useful per se, but it can understand and parse JSON text. Will be adding more useful infos soon!
My goal here is to implement a JSON parser from scratch just by referring this [JSON RFC](https://tools.ietf.org/html/rfc7159).

## Example:
```rust
// This example test demonstrates the basic functionalities of `rson`.
#[test]
fn test_parse() {
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

    let actual = Rson::from_reader(object.as_bytes());

    let mut map = HashMap::new();
    map.insert(
        r#""Id""#.to_string(),
        Value::Number(Number::new("93638382".to_string())),
    );
    map.insert(
        r#""Name""#.to_string(),
        Value::String(r#""Devajit Asem""#.to_string()),
    );
    map.insert(
        r#""HasGPU""#.to_string(),
        Value::Literal(Literal::Bool(true)),
    );
    map.insert(r#""Got3080""#.to_string(), Value::Literal(Literal::Null));
    map.insert(
        r#""CanAfford3090""#.to_string(),
        Value::Literal(Literal::Bool(false)),
    );

    let mut inner_map = HashMap::new();
    inner_map.insert(
        r#""RamType""#.to_string(),
        Value::String(r#""DDR6""#.to_string()),
    );
    inner_map.insert(
        r#""SerialNum""#.to_string(),
        Value::Number(Number::new("12837982".to_string())),
    );
    map.insert(
        r#""GPUDetail""#.to_string(),
        Value::Object(RsonMap(inner_map)),
    );
    map.insert(
        r#""Array""#.to_string(),
        Value::Array(vec![
            Value::String(r#""Devajit Asem""#.to_string()),
            Value::Number(Number::new("12324".to_string())),
            Value::Literal(Literal::Bool(true)),
            Value::Literal(Literal::Bool(false)),
            Value::Literal(Literal::Null),
        ]),
    );

    assert_eq!(actual, Value::Object(RsonMap(map)));
}
```
Currently, `rson` supports limited set of functionalities:  
- [x] Parse basic JSON structure
- [x] Parse literals: true, false, null
- [x] Parse basic number
- [x] Parse unescaped strings
- [x] Parse array
- [ ] Parse Decimal, Exponent numbers
- [ ] Parse escaped strings
- [ ] Support serialization
- [ ] Support deserialization to structs

## JSON Grammar(based on [RFC](https://tools.ietf.org/html/rfc7159)):

    JSON-text = ws value ws
    
    ws = *( SPACE | TAB | LINE_FEED/NEW_LINE | CR )
    
    Structural Characters:
        begin-array = ws [ ws
        begin-object = ws { ws
        end-array = ws ] ws
        end-object = ws } ws
        name-separator = ws : ws
        value-seperator = ws , ws
    
    Values:
        Value = (false | null | true | object | array | number | string)

    Objects:
        Object = begin-object [ member *( value-seperator member ) ]
                 end- object

        member = string name-seperator value

    Arrays:
        array = begin-array [ value *( value-seperator value ) ] end-array

    Numbers:
        number = [ minus ] int [ frac ] [ exp ]

        decimal-point = .

        digit1-9 = 1-9

        e = (e | E) ; lowercase | uppercase

        exp = e [ minus | plus ] 1*DIGIT

        frac = decimal-point 1*DIGIT

        int = zero / (digit1-9 *DIGIT)

        minus = -

        plus = +

        zero = 0

    Strings:
        string = quotation-mark *char quotation-mark
        char = unescaped | escape ( " | \ | / | b | f | n | r | t | uXXXX )
        escape = \
        quotation-mark = "
        unescaped = a-z | A-Z | %x5D-10FFFF
