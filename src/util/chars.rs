use crate::constants::characters::{DIGIT_REGEX, ID_REGEX, ID_START_REGEX};
use crate::constants::keywords::KEYWORDS;
use crate::constants::operators::{OPERATOR_TYPES};
use crate::constants::punctuations::PUNCTUATION_CHARACTERS;
use regex::Regex;

pub fn is_x(ch: u8, to: char) -> bool {
    ch.eq(&(to as u8))
}

pub fn is_whitespace(ch: u8) -> bool {
    is_x(ch, ' ')
}

pub fn is_newline(ch: u8) -> bool {
    is_x(ch, '\n')
}

pub fn is_carriage_return(ch: u8) -> bool {
    is_x(ch, '\r')
}

pub fn is_digit(ch: u8) -> bool {
    Regex::new(DIGIT_REGEX)
        .unwrap()
        .is_match(&String::from(ch as char))
}

pub fn is_id_start(ch: u8) -> bool {
    Regex::new(ID_START_REGEX)
        .unwrap()
        .is_match(&String::from(ch as char))
}

pub fn is_id(ch: u8) -> bool {
    Regex::new(ID_REGEX)
        .unwrap()
        .is_match(&String::from(ch as char))
}

pub fn is_indent(ch: u8) -> bool {
    is_x(ch, '\t')
}

pub fn is_skippable_char(ch: u8) -> bool {
    is_newline(ch) || is_carriage_return(ch) || is_whitespace(ch) || is_indent(ch)
}

pub fn is_quote(ch: u8) -> bool {
    is_x(ch, '"')
}

pub fn is_op(ch: u8) -> bool {
    OPERATOR_TYPES.contains(ch as char)
}

pub fn is_kw(str: &str) -> bool {
    KEYWORDS.contains(&str)
}

pub fn is_punc(ch: u8) -> bool {
    PUNCTUATION_CHARACTERS.contains(ch as char)
}
