// Copyright (c) 2024 Murilo Ijanc' <mbsd@m0x.ru>
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

use std::fmt;

pub const EOF: Token = Token::Eof;

// #[derive(Debug)]
// pub(crate) struct Token {
//     tp: TokenType,
//     // literal: &'a str
// }

// impl Token {
//     // pub fn new(tp: TokenType, literal: &'a str) -> Self {
//     //     Self {tp, literal}
//     // }

//     pub fn new(tp: TokenType) -> Self {
//         Self { tp }
//     }

//     pub fn tp(&self) -> &TokenType {
//         &self.tp
//     }

// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Token {
    Illegal(String),
    Eof,

    // Identifiers + literal
    Ident(String), // add, foobar, x, y, ...
    Int(String),   // 123456

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,

    // Delimiters
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
    Eq,
    NotEq,
}

impl Token {
    pub fn literal(&self) -> &str {
        self.as_str()
    }

    pub(crate) fn as_str(&self) -> &str {
        match self {
            Self::Illegal(s) => s.as_str(),
            Self::Eof => "",
            Self::Ident(s) => s.as_str(),
            Self::Int(s) => s.as_str(),
            Self::Assign => "=",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Bang => "!",
            Self::Asterisk => "*",
            Self::Slash => "/",
            Self::Lt => "<",
            Self::Gt => ">",
            Self::Comma => ",",
            Self::Semicolon => ";",
            Self::Lparen => "(",
            Self::Rparen => ")",
            Self::Lbrace => "{",
            Self::Rbrace => "}",
            Self::Function => "fn",
            Self::Let => "let",
            Self::True => "true",
            Self::False => "false",
            Self::If => "if",
            Self::Else => "else",
            Self::Return => "return",
            Self::Eq => "==",
            Self::NotEq => "!=",
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Illegal(s) => write!(f, "illegal({s})"),
            Self::Eof => write!(f, ""),
            Self::Ident(s) => write!(f, "ident({s})"),
            Self::Int(s) => write!(f, "int({s})"),
            Self::Assign => write!(f, "="),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Bang => write!(f, "!"),
            Self::Asterisk => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Lt => write!(f, "<"),
            Self::Gt => write!(f, ">"),
            Self::Comma => write!(f, ","),
            Self::Semicolon => write!(f, ";"),
            Self::Lparen => write!(f, "("),
            Self::Rparen => write!(f, ")"),
            Self::Lbrace => write!(f, "{{"),
            Self::Rbrace => write!(f, "}}"),
            Self::Function => write!(f, "fn"),
            Self::Let => write!(f, "let"),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::If => write!(f, "if"),
            Self::Else => write!(f, "else"),
            Self::Return => write!(f, "return"),
            Self::Eq => write!(f, "=="),
            Self::NotEq => write!(f, "!="),
        }
    }
}
