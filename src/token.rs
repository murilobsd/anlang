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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum TokenType {
    Illegal,
    Eof,

    // Identifiers + literal
    Ident, // add, foobar, x, y, ...
    Int,   // 123456

    // Operators
    Assign,
    Plus,

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
}

impl TokenType {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            TokenType::Illegal => "Illegal",
            TokenType::Eof => "",
            TokenType::Ident => "Ident",
            TokenType::Int => "Int",
            TokenType::Assign => "=",
            TokenType::Plus => "+",
            TokenType::Comma => ",",
            TokenType::Semicolon => ";",
            TokenType::Lparen => "(",
            TokenType::Rparen => ")",
            TokenType::Lbrace => "{",
            TokenType::Rbrace => "}",
            TokenType::Function => "Function",
            TokenType::Let => "Let",
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Illegal => write!(f, "Illegal"),
            Self::Eof => write!(f, ""),
            Self::Ident => write!(f, "Ident"),
            Self::Int => write!(f, "Int"),
            Self::Assign => write!(f, "="),
            Self::Plus => write!(f, "+"),
            Self::Comma => write!(f, ","),
            Self::Semicolon => write!(f, ";"),
            Self::Lparen => write!(f, "("),
            Self::Rparen => write!(f, ")"),
            Self::Lbrace => write!(f, "{{"),
            Self::Rbrace => write!(f, "}}"),
            Self::Function => write!(f, "Function"),
            Self::Let => write!(f, "Let"),
        }
    }
}
