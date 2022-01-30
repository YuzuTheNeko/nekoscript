use crate::constants::operators::SUB;
use crate::core::data_types::DataTypes;
use crate::core::nodes::{Node, Nodes, Position};
use crate::parsers::parse_array::parse_array;
use crate::parsers::parse_expression::parse_expression;
use crate::parsers::parse_id::parse_id;
use crate::parsers::parse_number::parse_number;
use crate::parsers::parse_op::parse_op;
use crate::parsers::parse_punc::parse_punc;
use crate::parsers::parse_string::parse_string;
use crate::util::chars::{is_comment, is_digit, is_id_start, is_newline, is_op, is_punc, is_quote, is_skippable_char, is_whitespace};

#[derive(Default)]
pub struct TokenStream<'a> {
    current: usize,
    column: usize,
    line: usize,

    path: String,

    code: &'a [u8],
    pub nodes: Vec<Node>,
}

pub type ReaderFn<'a> = &'a dyn Fn(u8) -> bool;

impl<'a> TokenStream<'a> {
    pub fn new(str: &'a str, path: String) -> Self {
        Self {
            line: 1,
            code: str.as_bytes(),
            path,
            ..Self::default()
        }
    }

    pub fn skip_useless(&mut self) -> String {
        if !is_skippable_char(self.char()) {
            return String::new()
        }

        self.read_while(&is_skippable_char)
    }

    pub fn read_while(&mut self, f: ReaderFn) -> String {
        let mut str = String::new();

        while !self.eof() && f(self.char()) {
            str.push(self.forward() as char)
        }

        str
    }

    pub fn forward(&mut self) -> u8 {
        let char = self.char();

        if is_newline(char) {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }

        self.current += 1;

        char
    }

    pub fn str(&mut self, size: usize) -> String {
        self.skip_useless();

        let mut str = String::new();

        let mut i = 0;

        while !self.eof() && str.len() != size {
            str.push(self.at(i) as char);
            i += 1;
        }

        str
    }

    pub fn panic(&self, str: &str) {
        println!("{} at {}:{}", &str, self.line, self.column);
        std::process::exit(0);
    }

    pub fn at(&self, offset: usize) -> u8 {
        let pos = offset + self.current;
        if pos >= self.code.len() {
            0
        } else {
            self.code[pos]
        }
    }

    pub fn eof(&self) -> bool {
        self.current == self.code.len()
    }

    pub fn pos(&mut self) -> Position {
        self.skip_useless();

        Position {
            line: self.line,
            column: self.column
        }
    }

    pub fn peek(&mut self) -> Node {
        if self.eof() {
            Nodes::create(Nodes::Value(DataTypes::null()), self.pos())
        } else {
            let oldc = self.current.clone();
            let oldcl = self.column.clone();
            let oldln = self.line.clone();

            let node = self.read_next();

            self.current = oldc;
            self.column = oldcl;
            self.line = oldln;

            node
        }
    }

    pub fn skip_kw(&mut self, kw: &str) {
        let str = self.str(kw.len());
        if !kw.eq(&str) {
            self.panic(&format!("Unexpected token {}, wanted {}", str, kw));
            unreachable!()
        }
        self.skip(kw.len())
    }

    pub fn skip_punc(&mut self, char: char) {
        if !self.is_punc(char) {
            self.panic(&format!(
                "Unexpected token {}, wanted {}",
                self.char() as char,
                char
            ))
        }
        self.skip(1)
    }

    pub fn skip_comment(&mut self) {
        self.read_while(&| n | !is_newline(n));
    }

    pub fn skip_op(&mut self, op: &str) {
        self.skip_kw(&op)
    }

    pub fn read_next(&mut self) -> Node {
        self.skip_useless();

        let char = self.char();

        if char.eq(&0) {
            return Nodes::create(Nodes::Value(DataTypes::null()), self.pos());
        }

        if is_comment(char) {
            self.skip_comment();
            return self.read_next()
        } else if is_op(char) {
            return parse_op(self);
        } else if char.eq(&b'[') {
            return parse_array(self);
        } else if is_id_start(char) {
            return parse_id(self);
        } else if is_digit(char) || self.is_op(SUB) {
            return parse_number(self);
        } else if is_quote(char) {
            return parse_string(self);
        } else if is_punc(char) {
            return parse_punc(self);
        }

        self.panic(&format!("Unexpected token {}", char as char));
        unreachable!()
    }

    pub fn is_kw(&mut self, kw: &str) -> bool {
        self.str(kw.len()).eq(&kw)
    }

    pub fn is_punc(&mut self, ch: char) -> bool {
        self.skip_useless();
        self.char().eq(&(ch as u8))
    }

    pub fn is_op(&mut self, ch: &str) -> bool {
        self.str(ch.len()).eq(&ch)
    }

    pub fn skip(&mut self, len: usize) {
        let mut len = len;

        while len != 0 {
            self.forward();
            len -= 1;
        }
    }

    pub fn start(&mut self) {
        while !self.eof() {
            let expr = parse_expression(self);

            self.nodes.push(expr);

            if !self.eof() {
                self.skip_punc(';')
            }
        }
    }

    pub fn fake_read_while(&mut self, f: ReaderFn) -> String {
        self.skip_useless();

        let mut str = String::new();

        let mut i = 0;

        while !self.eof() && f(self.at(i)) {
            str.push(self.at(i) as char);
            i += 1;
        }

        str
    }

    pub fn char(&self) -> u8 {
        if self.eof() {
            0
        } else {
            self.code[self.current]
        }
    }
}
