use std::{fmt, str};

use crate::{Obj, Val};

#[derive(Default)]
pub struct Parser<'a> {
    bytes: &'a [u8],
    line: usize,
    index_of_last_line: usize,
    index: usize,
}

pub struct ParserErr {
    msg: String,
    line: usize,
    line_index: usize,
    index: usize,
}

impl fmt::Display for ParserErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ParserErr {
    pub fn new(msg: String, parser: &Parser) -> ParserErr {
        ParserErr {
            msg,
            line: parser.line + 1,
            line_index: parser.index - parser.index_of_last_line,
            index: parser.index,
        }
    }

    pub fn unexpected(expected: &str, parser: &Parser) -> Result<Val, ParserErr> {
        let Parser { index, bytes, .. } = parser;
        let end = if index + 5 < bytes.len() {
            index + 5
        } else {
            index + 1
        };
        let found = &bytes[*index..end];

        let msg = format!(
            "Unexpected value, expect: {}, but found '{}'.",
            expected,
            str::from_utf8(found).unwrap_or("unknown")
        );
        Err(ParserErr::new(msg, parser))
    }

    pub fn never_ended(
        val_type: &str,
        start_at_line: usize,
        start_at_char: usize,
        parser: &Parser,
    ) -> Result<Val, ParserErr> {
        let msg = format!(
            "{} start at {}, {}, never ended.",
            val_type, start_at_line, start_at_char
        );
        Err(ParserErr::new(msg, parser))
    }
}

impl fmt::Debug for ParserErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ParserErr {
            line,
            line_index,
            msg,
            index,
        } = self;
        write!(
            f,
            "{} at line ({}, {}) char number {}",
            msg, line, line_index, index
        )
    }
}

impl Parser<'_> {
    pub fn new(str: &str) -> Parser {
        Parser {
            bytes: str.as_bytes(),
            ..Parser::default()
        }
    }

    pub fn parse(&mut self, check_else: bool) -> Result<Val, ParserErr> {
        self.continue_empty();
        let c: char = self.bytes[self.index].into();
        let result = match c {
            '\"' => self.parse_string(),
            'n' => self.parse_null(),
            't' | 'f' => self.parse_boolean(),
            c if c.is_numeric() => self.parse_number(),
            '{' => self.parse_object(),
            '[' => self.parse_array(),
            _ => ParserErr::unexpected("any json value", self),
        };
        if check_else && self.is_there_more() {
            if let Ok(_) = result {
                ParserErr::unexpected("nothing", self)
            } else {
                result
            }
        } else {
            result
        }
    }

    pub fn parse_string(&mut self) -> Result<Val, ParserErr> {
        let mut result_str = String::new();
        self.continue_empty();
        if !self.is_index_match("\"") {
            return ParserErr::unexpected("'\"'", self);
        }

        let start_at_char = self.index - self.index_of_last_line;
        let start_at_line = self.line;
        self.advance_index();

        for i in self.index..self.bytes.len() {
            let c = self.bytes[i].into();
            self.index = i;

            match c {
                c if c == '"' && !result_str.ends_with("\\") => {
                    self.advance_index();
                    return Ok(Val::Str(result_str));
                }
                '\n' => {
                    self.new_line();
                    result_str.push(c);
                }
                _ => result_str.push(c),
            }
        }
        ParserErr::never_ended("String", start_at_line, start_at_char, self)
    }
    pub fn parse_number(&mut self) -> Result<Val, ParserErr> {
        let mut result = String::new();
        for i in self.index..self.bytes.len() {
            let c: char = self.bytes[i].into();
            self.index = i;

            if c.is_numeric() {
                result.push(c);
            } else {
                break;
            }
        }
        if result.len() > 0 {
            Ok(Val::Num(result.parse().unwrap()))
        } else {
            ParserErr::unexpected("any number", self)
        }
    }
    pub fn parse_boolean(&mut self) -> Result<Val, ParserErr> {
        if self.is_index_match("true") {
            self.index += 4;
            Ok(Val::Bool(true))
        } else if self.is_index_match("false") {
            self.index += 5;
            Ok(Val::Bool(false))
        } else {
            ParserErr::unexpected("'true' or 'false'", self)
        }
    }
    pub fn parse_null(&mut self) -> Result<Val, ParserErr> {
        if self.is_index_match("null") {
            self.index += 4;
            Ok(Val::Null)
        } else {
            ParserErr::unexpected("'null' or any json value", self)
        }
    }

    pub fn parse_object(&mut self) -> Result<Val, ParserErr> {
        let mut result = Obj::new();
        self.continue_empty();
        if !self.is_index_match("{") {
            return ParserErr::unexpected("'{'", self);
        }

        let start_at_char = self.index - self.index_of_last_line;
        let start_at_line = self.line;
        self.advance_index();
        self.continue_empty();
        // if json is empty
        if self.is_index_match("}") {
            self.advance_index();
            return Ok(Val::Obj(result));
        }

        let mut i = self.index;
        while i < self.bytes.len() {
            self.continue_empty();
            let _key = self.parse_string();
            let key;
            if let Ok(Val::Str(v)) = _key {
                key = v;
            } else {
                return _key;
            }

            self.continue_empty();
            if !self.is_index_match(":") {
                return Err(ParserErr::new(format!("Expected ':'"), self));
            }

            self.advance_index();

            let val = self.parse(false);

            if let Ok(v) = val {
                result.set(key.as_str(), v);
            } else {
                return val;
            }
            self.continue_empty();
            if self.is_index_match(",") {
                self.advance_index();
                i = self.index;
                continue;
            } else if self.is_index_match("}") {
                self.advance_index();
                return Ok(Val::Obj(result));
            } else {
                return ParserErr::unexpected("',' or '}'", self);
            }
        }
        ParserErr::never_ended("Object", start_at_line, start_at_char, self)
    }

    pub fn parse_array(&mut self) -> Result<Val, ParserErr> {
        let mut result = vec![];
        self.continue_empty();
        if !self.is_index_match("[") {
            return ParserErr::unexpected("'['", self);
        }

        let start_at_char = self.index - self.index_of_last_line;
        let start_at_line = self.line;
        self.advance_index();
        self.continue_empty();
        // if array is empty
        if self.is_index_match("]") {
            self.advance_index();
            return Ok(Val::Array(result));
        }
        let mut i = self.index;
        while i < self.bytes.len() {
            let val = self.parse(false);
            if let Ok(v) = val {
                result.push(v);
            }
            self.continue_empty();
            if self.is_index_match(",") {
                self.advance_index();
                i = self.index;
            } else if self.is_index_match("]") {
                self.advance_index();
                return Ok(Val::Array(result));
            } else {
                return ParserErr::unexpected("',' or ']'", self);
            }
        }
        ParserErr::never_ended("Array", start_at_line, start_at_char, self)
    }

    fn is_there_more(&mut self) -> bool {
        self.continue_empty();
        self.index + 1 != self.bytes.len()
    }

    fn new_line(&mut self) {
        self.index_of_last_line = self.index;
        self.line += 1;
    }

    fn is_index_match(&self, c: &str) -> bool {
        let Parser { bytes, index, .. } = self;
        if bytes.len() < c.len() + *index {
            return false;
        }
        let content: &str = str::from_utf8(&bytes[*index..(*index + c.len())]).unwrap();
        content == c
    }

    fn advance_index(&mut self) -> bool {
        if self.bytes.len() > self.index + 1 {
            self.index += 1;
            true
        } else {
            false
        }
    }

    fn continue_empty(&mut self) -> usize {
        let mut steps = 0;
        for i in self.index..self.bytes.len() {
            let c: char = self.bytes[i].into();
            self.index = i;
            match c {
                '\n' => {
                    self.new_line();
                    steps += 1;
                }
                '\r' | '\t' | ' ' => steps += 1,
                _ => break,
            }
        }
        steps
    }
}
