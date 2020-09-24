# rson
A minimal JSON parser in Rust

## JSON Grammar:

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

Currently, `rson` supports limited set of functionalities:  
- [x] Parse basic JSON structure
- [x] Parse literals: true, false, null
- [x] Parse basic number
- [x] Parse unescaped strings
- [ ] Parse array
- [ ] Parse Decimal, Exponent numbers
- [ ] Parse escaped strings
