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

pub(crate) struct Token {
    tp: TokenType,
    // literal: &'a str
}

impl Token {
    // pub fn new(tp: TokenType, literal: &'a str) -> Self {
    //     Self {tp, literal}
    // }

    pub fn new(tp: TokenType) -> Self {
        Self { tp }
    }

    pub fn tp(&self) -> &TokenType {
        &self.tp
    }

    pub fn literal(&self) -> &str {
        self.tp.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TokenType {
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

impl TokenType {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            TokenType::Illegal(s) => s.as_str(),
            TokenType::Eof => "",
            TokenType::Ident(s) => s.as_str(),
            TokenType::Int(s) => s.as_str(),
            TokenType::Assign => "=",
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Bang => "!",
            TokenType::Asterisk => "*",
            TokenType::Slash => "/",
            TokenType::Lt => "<",
            TokenType::Gt => ">",
            TokenType::Comma => ",",
            TokenType::Semicolon => ";",
            TokenType::Lparen => "(",
            TokenType::Rparen => ")",
            TokenType::Lbrace => "{",
            TokenType::Rbrace => "}",
            TokenType::Function => "fn",
            TokenType::Let => "let",
            TokenType::True => "true",
            TokenType::False => "false",
            TokenType::If => "if",
            TokenType::Else => "else",
            TokenType::Return => "return",
            TokenType::Eq => "==",
            TokenType::NotEq => "!=",
        }
    }
}

impl fmt::Display for TokenType {
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
