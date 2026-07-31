//! A complete JSON parser + printer in one file.
//! text -> Value  (parse)      Value -> text  (pretty / compact)

use std::fmt;

// ============================================================
//  THE TYPE  (Program 4)
// ============================================================

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<Value>),
    Object(Vec<(String, Value)>), // Vec, not HashMap: preserves key order
}

// ============================================================
//  ERRORS
// ============================================================

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub pos: usize, // character index into the input
    pub msg: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "at char {}: {}", self.pos, self.msg)
    }
}

impl std::error::Error for ParseError {}

/// Render the error with a caret under the offending character.
pub fn explain(input: &str, e: &ParseError) -> String {
    let chars: Vec<char> = input.chars().collect();
    let line_start = chars[..e.pos.min(chars.len())]
        .iter()
        .rposition(|&c| c == '\n')
        .map(|i| i + 1)
        .unwrap_or(0);
    let line_end = chars[e.pos.min(chars.len())..]
        .iter()
        .position(|&c| c == '\n')
        .map(|i| e.pos + i)
        .unwrap_or(chars.len());
    let line: String = chars[line_start..line_end].iter().collect();
    let caret = " ".repeat(e.pos - line_start) + "^";
    format!("{}\n  {}\n  {}", e, line, caret)
}

// ============================================================
//  PARSER   text -> Value
// ============================================================

pub struct Parser {
    chars: Vec<char>,
    pos: usize,
}

/// Parse a whole document. Rejects trailing junk.
pub fn parse(input: &str) -> Result<Value, ParseError> {
    let mut p = Parser::new(input);
    let v = p.parse_value()?;
    p.skip_ws();
    if p.pos != p.chars.len() {
        return Err(ParseError {
            pos: p.pos,
            msg: "trailing characters after value".into(),
        });
    }
    Ok(v)
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Parser {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    // ---- the three primitives everything else is built from ----
    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }
    fn bump(&mut self) -> Option<char> {
        let c = self.peek();
        if c.is_some() {
            self.pos += 1;
        }
        c
    }
    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(' ' | '\n' | '\t' | '\r')) {
            self.pos += 1;
        }
    }

    fn err<T>(&self, msg: impl Into<String>) -> Result<T, ParseError> {
        Err(ParseError {
            pos: self.pos,
            msg: msg.into(),
        })
    }

    fn expect(&mut self, want: char) -> Result<(), ParseError> {
        match self.peek() {
            Some(got) if got == want => {
                self.pos += 1;
                Ok(())
            }
            Some(got) => self.err(format!("expected '{}', found '{}'", want, got)),
            None => self.err(format!("expected '{}', found end of input", want)),
        }
    }

    // ---- THE DISPATCH: one character decides everything ----
    pub fn parse_value(&mut self) -> Result<Value, ParseError> {
        self.skip_ws();
        match self.peek() {
            Some('{') => self.parse_object(),
            Some('[') => self.parse_array(),
            Some('"') => Ok(Value::Str(self.parse_string()?)),
            Some('t') => self.literal("true", Value::Bool(true)),
            Some('f') => self.literal("false", Value::Bool(false)),
            Some('n') => self.literal("null", Value::Null),
            Some(c) if c == '-' || c.is_ascii_digit() => self.parse_number(),
            Some(c) => self.err(format!("unexpected character '{}'", c)),
            None => self.err("unexpected end of input"),
        }
    }

    fn parse_object(&mut self) -> Result<Value, ParseError> {
        self.expect('{')?;
        let mut fields = Vec::new();

        self.skip_ws();
        if self.peek() == Some('}') {
            self.pos += 1;
            return Ok(Value::Object(fields));
        }

        loop {
            self.skip_ws();
            let key = self.parse_string()?; // keys must be strings
            self.skip_ws();
            self.expect(':')?;
            let val = self.parse_value()?; // <-- recursion: nesting is free
            fields.push((key, val));

            self.skip_ws();
            match self.bump() {
                Some(',') => continue,
                Some('}') => break,
                Some(c) => {
                    self.pos -= 1;
                    return self.err(format!("expected ',' or '}}', found '{}'", c));
                }
                None => return self.err("expected ',' or '}', found end of input"),
            }
        }
        Ok(Value::Object(fields))
    }

    fn parse_array(&mut self) -> Result<Value, ParseError> {
        self.expect('[')?;
        let mut items = Vec::new();

        self.skip_ws();
        if self.peek() == Some(']') {
            self.pos += 1;
            return Ok(Value::Array(items));
        }

        loop {
            items.push(self.parse_value()?); // <-- recursion again

            self.skip_ws();
            match self.bump() {
                Some(',') => continue,
                Some(']') => break,
                Some(c) => {
                    self.pos -= 1;
                    return self.err(format!("expected ',' or ']', found '{}'", c));
                }
                None => return self.err("expected ',' or ']', found end of input"),
            }
        }
        Ok(Value::Array(items))
    }

    /// The ONLY function that looks at characters between quotes.
    /// This is why commas/colons inside strings never confuse the loops above.
    fn parse_string(&mut self) -> Result<String, ParseError> {
        self.expect('"')?;
        let mut out = String::new();

        loop {
            match self.bump() {
                None => return self.err("unterminated string"),
                Some('"') => return Ok(out),
                Some('\\') => match self.bump() {
                    Some('"') => out.push('"'),
                    Some('\\') => out.push('\\'),
                    Some('/') => out.push('/'),
                    Some('b') => out.push('\u{08}'),
                    Some('f') => out.push('\u{0c}'),
                    Some('n') => out.push('\n'),
                    Some('r') => out.push('\r'),
                    Some('t') => out.push('\t'),
                    Some('u') => out.push(self.parse_unicode_escape()?),
                    Some(c) => return self.err(format!("invalid escape '\\{}'", c)),
                    None => return self.err("unterminated escape"),
                },
                Some(c) if (c as u32) < 0x20 => {
                    return self.err("raw control character in string (must be escaped)");
                }
                Some(c) => out.push(c),
            }
        }
    }

    /// \uXXXX, including surrogate pairs for astral characters (emoji).
    fn parse_unicode_escape(&mut self) -> Result<char, ParseError> {
        let hi = self.hex4()?;
        if (0xD800..=0xDBFF).contains(&hi) {
            self.expect('\\')?;
            self.expect('u')?;
            let lo = self.hex4()?;
            if !(0xDC00..=0xDFFF).contains(&lo) {
                return self.err("expected low surrogate");
            }
            let cp = 0x10000 + ((hi - 0xD800) << 10) + (lo - 0xDC00);
            char::from_u32(cp).map_or_else(|| self.err("invalid code point"), Ok)
        } else {
            char::from_u32(hi).map_or_else(|| self.err("invalid code point"), Ok)
        }
    }

    fn hex4(&mut self) -> Result<u32, ParseError> {
        let mut v = 0u32;
        for _ in 0..4 {
            let c = match self.bump() {
                Some(c) => c,
                None => return self.err("unexpected end in \\u escape"),
            };
            match c.to_digit(16) {
                Some(d) => v = v * 16 + d,
                None => {
                    self.pos -= 1;
                    return self.err(format!("invalid hex digit '{}'", c));
                }
            }
        }
        Ok(v)
    }

    fn parse_number(&mut self) -> Result<Value, ParseError> {
        let start = self.pos;
        if self.peek() == Some('-') {
            self.pos += 1;
        }

        // integer part: '0' alone, or [1-9] followed by digits
        match self.peek() {
            Some('0') => self.pos += 1,
            Some(c) if c.is_ascii_digit() => {
                while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
                    self.pos += 1;
                }
            }
            _ => return self.err("expected a digit"),
        }
        if matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
            return self.err("leading zeros are not allowed in JSON numbers");
        }

        // fraction
        if self.peek() == Some('.') {
            self.pos += 1;
            if !matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
                return self.err("expected a digit after '.'");
            }
            while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
                self.pos += 1;
            }
        }

        // exponent
        if matches!(self.peek(), Some('e' | 'E')) {
            self.pos += 1;
            if matches!(self.peek(), Some('+' | '-')) {
                self.pos += 1;
            }
            if !matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
                return self.err("expected a digit in the exponent");
            }
            while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
                self.pos += 1;
            }
        }

        let text: String = self.chars[start..self.pos].iter().collect();
        match text.parse::<f64>() {
            Ok(n) => Ok(Value::Number(n)),
            Err(_) => Err(ParseError {
                pos: start,
                msg: format!("invalid number '{}'", text),
            }),
        }
    }

    fn literal(&mut self, word: &str, val: Value) -> Result<Value, ParseError> {
        let start = self.pos;
        for want in word.chars() {
            match self.bump() {
                Some(c) if c == want => {}
                _ => {
                    return Err(ParseError {
                        pos: start,
                        msg: format!("expected '{}'", word),
                    });
                }
            }
        }
        Ok(val)
    }
}

// ============================================================
//  PRINTERS   Value -> text
// ============================================================

fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0c}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

fn number_to_string(n: f64) -> String {
    if n.is_finite() {
        n.to_string()
    } else {
        "null".to_string()
    } // JSON has no NaN/Infinity
}

/// One line, no spaces.
pub fn compact(v: &Value) -> String {
    match v {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => number_to_string(*n),
        Value::Str(s) => format!("\"{}\"", escape(s)),
        Value::Array(items) => {
            let parts: Vec<String> = items.iter().map(compact).collect();
            format!("[{}]", parts.join(","))
        }
        Value::Object(fields) => {
            let parts: Vec<String> = fields
                .iter()
                .map(|(k, val)| format!("\"{}\":{}", escape(k), compact(val)))
                .collect();
            format!("{{{}}}", parts.join(","))
        }
    }
}

/// Indented, one entry per line, no trailing commas.
pub fn pretty(v: &Value) -> String {
    let mut out = String::new();
    write_pretty(v, 0, &mut out);
    out
}

fn write_pretty(v: &Value, depth: usize, out: &mut String) {
    let pad = "  ".repeat(depth);
    let pad_in = "  ".repeat(depth + 1);

    match v {
        Value::Array(items) if !items.is_empty() => {
            out.push_str("[\n");
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push_str(",\n");
                } // separator BEFORE, not after -> no trailing comma
                out.push_str(&pad_in);
                write_pretty(item, depth + 1, out);
            }
            out.push('\n');
            out.push_str(&pad);
            out.push(']');
        }
        Value::Object(fields) if !fields.is_empty() => {
            out.push_str("{\n");
            for (i, (k, val)) in fields.iter().enumerate() {
                if i > 0 {
                    out.push_str(",\n");
                }
                out.push_str(&pad_in);
                out.push('"');
                out.push_str(&escape(k));
                out.push_str("\": ");
                write_pretty(val, depth + 1, out);
            }
            out.push('\n');
            out.push_str(&pad);
            out.push('}');
        }
        // scalars, [] and {} are identical in both modes
        scalar => out.push_str(&compact(scalar)),
    }
}

// ============================================================
//  DEMO + TESTS
// ============================================================

fn main() {
    let doc = r#"
{
  "a": "one",
  "b": ["one", "two"],
  "c": ["1", "2", 3],
  "d": {
    "a": "two",
    "b": "three"
  },
  "e": [1, 2],
  "f": null,
  "g": true,
  "h": {"deep": {"deeper": [[], {}, "x, y: z"]}},
  "i": "quote:\" newline:\n emoji:🦀"
}"#;

    let v = parse(doc).expect("should parse");

    println!("=== PRETTY ===\n{}", pretty(&v));
    println!("\n=== COMPACT ===\n{}", compact(&v));

    // round-trip: printing then re-parsing must give the identical value
    assert_eq!(parse(&pretty(&v)).unwrap(), v);
    assert_eq!(parse(&compact(&v)).unwrap(), v);
    println!("\n=== ROUND-TRIP OK (pretty and compact both re-parse to the same Value) ===");

    println!("\n=== ERRORS ===");
    for bad in [
        r#"{"a": "two" "b": "three"}"#, // your example: missing comma
        r#"{"a": [1, 2,]}"#,            // trailing comma
        r#"{"a": "unterminated}"#,
        r#"{"a": 1} extra"#,
        r#"{a: 1}"#, // unquoted key
    ] {
        match parse(bad) {
            Ok(_) => println!("!! accepted bad input: {}", bad),
            Err(e) => println!("{}\n", explain(bad, &e)),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_cv() {
        let cv = Value::Object(vec![
            ("name".into(), Value::Str("Subash".into())),
            (
                "skills".into(),
                Value::Array(vec![Value::Str("rust".into()), Value::Str("sql".into())]),
            ),
            (
                "address".into(),
                Value::Object(vec![
                    ("city".into(), Value::Str("NYC".into())),
                    ("zip".into(), Value::Str("10001".into())),
                ]),
            ),
        ]);
        let expected = "\
{
  \"name\": \"Subash\",
  \"skills\": [
    \"rust\",
    \"sql\"
  ],
  \"address\": {
    \"city\": \"NYC\",
    \"zip\": \"10001\"
  }
}";
        assert_eq!(pretty(&cv), expected);
        assert_eq!(
            compact(&cv),
            "{\"name\":\"Subash\",\"skills\":[\"rust\",\"sql\"],\"address\":{\"city\":\"NYC\",\"zip\":\"10001\"}}"
        );
    }

    #[test]
    fn empty_containers() {
        assert_eq!(pretty(&parse("[]").unwrap()), "[]");
        assert_eq!(pretty(&parse("{}").unwrap()), "{}");
        assert_eq!(pretty(&parse("[[],{}]").unwrap()), "[\n  [],\n  {}\n]");
    }

    #[test]
    fn scalars() {
        assert_eq!(parse("null").unwrap(), Value::Null);
        assert_eq!(parse("true").unwrap(), Value::Bool(true));
        assert_eq!(parse("-1.5e3").unwrap(), Value::Number(-1500.0));
        assert_eq!(parse(r#""A🦀""#).unwrap(), Value::Str("A🦀".into()));
    }

    #[test]
    fn commas_inside_strings_are_safe() {
        let v = parse(r#"{"a": "two, three", "b": "x: y"}"#).unwrap();
        assert_eq!(
            v,
            Value::Object(vec![
                ("a".into(), Value::Str("two, three".into())),
                ("b".into(), Value::Str("x: y".into())),
            ])
        );
    }

    #[test]
    fn round_trip() {
        let src = r#"{"a":[1,{"b":null},true],"c":"q\"uote"}"#;
        let v = parse(src).unwrap();
        assert_eq!(parse(&compact(&v)).unwrap(), v);
        assert_eq!(parse(&pretty(&v)).unwrap(), v);
        assert_eq!(compact(&v), src);
    }

    #[test]
    fn rejects_bad_input() {
        for bad in [
            r#"{"a": "two" "b": 1}"#,
            r#"[1,]"#,
            r#"{a:1}"#,
            r#"{"a":1} junk"#,
            r#""unterminated"#,
            r#"{"a":}"#,
            r#"[01]"#.trim(),
            r#"tru"#,
        ] {
            assert!(parse(bad).is_err(), "should have rejected: {}", bad);
        }
    }

    #[test]
    fn error_positions() {
        let e = parse(r#"{"a": "two" "b": 1}"#).unwrap_err();
        assert_eq!(e.pos, 12);
    }
}
