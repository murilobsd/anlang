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

pub const EOF: Token = Token::Eof;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Token {
    Assign,
    Asterisk,
    Bang,
    Comma,
    Else,
    Eof,
    Eq,
    False,
    Function,
    Gt,
    Ident(String),
    If,
    Illegal(String),
    Int(String),
    Lbrace,
    Let,
    Lparen,
    Lt,
    Minus,
    NotEq,
    Plus,
    Rbrace,
    Return,
    Rparen,
    Semicolon,
    Slash,
    True,
}

impl Token {
    pub fn literal(&self) -> &str {
        match self {
            Self::Assign => "=",
            Self::Asterisk => "*",
            Self::Bang => "!",
            Self::Comma => ",",
            Self::Else => "else",
            Self::Eof => "",
            Self::Eq => "==",
            Self::False => "false",
            Self::Function => "fn",
            Self::Gt => ">",
            Self::Ident(s) => s.as_str(),
            Self::If => "if",
            Self::Illegal(s) => s.as_str(),
            Self::Int(s) => s.as_str(),
            Self::Lbrace => "{",
            Self::Let => "let",
            Self::Lparen => "(",
            Self::Lt => "<",
            Self::Minus => "-",
            Self::NotEq => "!=",
            Self::Plus => "+",
            Self::Rbrace => "}",
            Self::Return => "return",
            Self::Rparen => ")",
            Self::Semicolon => ";",
            Self::Slash => "/",
            Self::True => "true",
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.literal())
    }
}
