# A Backus Naur Specification of MSBasic

This is a Backus Naur Specification of the MSBasic language.

```BNF
<PROGRAM> ::= <CODE-LINE> | <CODE-LINE> <PROGRAM>

<CODE-LINE> ::= <LINE-NUMBER> <WHITESPACE> <STATEMENTS>
    <OPTIONAL-WHITESPACE> <END-OF-LINE>

<LINE-NUMBER> ::= <INTEGER>

<DECIMAL-DIGITS> ::= <DECIMAL-DIGIT> | <DECIMAL-DIGIT> <DECIMAL-DIGITS>

<DECIMAL-DIGIT> ::= <OCTAL-DIGIT> | "8" | "9"

<OCTAL-NUMBER> ::= "&" <OCTAL-DIGITS> | "&O" <OCTAL-DIGITS>

<OCTAL-DIGITS> ::= <OCTAL-DIGIT> | <OCTAL-DIGIT> <OCTAL-DIGITS> 

<OCTAL-DIGIT> ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7"

<HEX-NUMBER> ::= "&H" | <HEX-DIGITS>

<HEX-DIGITS> ::= <HEX-DIGIT> | <HEX-DIGIT> <HEX-DIGITS>

<HEX-DIGIT> ::= <DECIMAL-DIGIT> | "A" | "B" | "C" | "D" | "E" | "F"

<SIGNED-INTEGER> ::= "-" <INTEGER> | "+" <INTEGER> | <INTEGER>

<INTEGER> ::= <DECIMAL-DIGITS>

<FIXED-POINT> ::= <SIGNED-INTEGER> "." <INTEGER>

<SINGLE-PRECISION> ::= <MANTISSA> "E" <EXPONENT> | <FIXED-POINT> "!"

<DOUBLE-PRECISION> ::= <MANTISSA> "D" <EXPONENT> | <FIXED-POINT> "#"

<MANTISSA> ::= <SIGNED-INTEGER>

<EXPONENT> ::= <SIGNED-INTEGER>

<STRING-CONSTANT> ::= '""' | '"' <MIDDLE-STRING> '"'

<MIDDLE-STRING> ::= <UPPER-CASE-LETTER>
    | <UPPER-CASE-LETTER> <MIDDLE-STRING>
    | <LOWER-CASE-LETTER>
    | <LOWER-CASE-LETTER> <MIDDLE-STRING>
    | <DECIMAL-DIGIT>
    | <DECIMAL-DIGIT> <MIDDLE-STRING>
    | <MIDDLE-STRING-PUNCTUATION>
    | <MIDDLE-STRING-PUNCTUATION> <MIDDLE-STRING>
    | <WHITESPACE>
    | <WHITESPACE> <MIDDLE-STRING>

<MIDDLE-STRING-PUNCTUATION> ::= "!" | "#" | "$" | "%" | "&" | "'"
    | "(" | ")" | "*" | "+" | "," | "-" | "." | "/" | ":" | ";"
    | "<" | "=" | ">" | "?" | "@" | "[" | "\" | "]" | "^" | "_"
    | "`" | "{" | "|" | "}" | "~"

<UPPER-CASE-LETTER> ::= "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H"
    | "I" | "J" | "K" | "L" | "M" | "N" | "O" | "P" | "Q" | "R" | "S"
    | "T" | "U" | "V" | "W" | "X" | "Y" | "Z"

<LOWER-CASE-LETTER> ::= "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h"
    | "i" | "j" | "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s"
    | "t" | "u" | "v" | "w" | "x" | "y" | "z"

<UPPER-CASE-LETTER-NO-F> ::= "A" | "B" | "C" | "D" | "E" | "G" | "H"
    | "I" | "J" | "K" | "L" | "M" | "N" | "O" | "P" | "Q" | "R" | "S"
    | "T" | "U" | "V" | "W" | "X" | "Y" | "Z"

<UPPER-CASE-LETTER-NO-N> ::= "A" | "B" | "C" | "D" | "E" | "F" | "G"
    | "H" | "I" | "J" | "K" | "L" | "M" | "O" | "P" | "Q" | "R" | "S"
    | "T" | "U" | "V" | "W" | "X" | "Y" | "Z"

<VARIABLE-NAME> ::= <SINGLE-LETTER-VARIABLE-NAME>
    | <MULTI-LETTER-VARIABLE-NAME>

<SINGLE-LETTER-VARIABLE-NAME> ::=
    <UPPER-CASE-LETTER> <TYPE-DECLARATION-CHARACTER>

<MULTI-LETTER-VARIABLE-NAME>
    <UPPER-CASE-LETTER-NO-F> <UPPER-CASE-LETTER-NO-N>
    <VARIABLE-NAME-MIDDLE> <TYPE-DECLARATION-CHARACTER>

<VARIABLE-NAME-MIDDLE> ::= "" | "." <VARIABLE-NAME-MIDDLE>
    | <UPPER-CASE-LETTER> <VARIABLE-NAME-MIDDLE>
    | <DECIMAL-DIGIT> <VARIABLE-NAME-MIDDLE>

<TYPE-DECLARATION-CHARACTER> ::= "$" | "%" | "!" | "#"

<WHITESPACE> ::= " " | "\t" | " " <WHITESPACE> | "\t" <WHITESPACE>

<STATEMENTS> ::= <STATEMENT> | <STATEMENT> ":"

<OPTIONAL-WHITESPACE> ::= <WHITESPACE> | ""

<END-OF-LINE>::= "\r\n"
```