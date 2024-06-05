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

#[derive(PartialEq, Eq)]
pub enum Token {
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

    fn debug_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        token_type: &str,
    ) -> std::fmt::Result {
        f.debug_struct("Token")
            .field("type", &token_type)
            .field("literal", &self.literal())
            .finish()
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.literal())
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Assign => self.debug_fmt(f, "assign"),
            Self::Asterisk => self.debug_fmt(f, "asterisk"),
            Self::Bang => self.debug_fmt(f, "bang"),
            Self::Comma => self.debug_fmt(f, "comma"),
            Self::Else => self.debug_fmt(f, "else"),
            Self::Eof => self.debug_fmt(f, "eof"),
            Self::Eq => self.debug_fmt(f, "eq"),
            Self::False => self.debug_fmt(f, "false"),
            Self::Function => self.debug_fmt(f, "function"),
            Self::Gt => self.debug_fmt(f, "gt"),
            Self::Ident(_) => self.debug_fmt(f, "ident"),
            Self::If => self.debug_fmt(f, "if"),
            Self::Illegal(_) => self.debug_fmt(f, "illegal"),
            Self::Int(_) => self.debug_fmt(f, "int"),
            Self::Lbrace => self.debug_fmt(f, "lbrace"),
            Self::Let => self.debug_fmt(f, "let"),
            Self::Lparen => self.debug_fmt(f, "lparen"),
            Self::Lt => self.debug_fmt(f, "lt"),
            Self::Minus => self.debug_fmt(f, "minus"),
            Self::NotEq => self.debug_fmt(f, "noteq"),
            Self::Plus => self.debug_fmt(f, "plus"),
            Self::Rbrace => self.debug_fmt(f, "rbrace"),
            Self::Return => self.debug_fmt(f, "return"),
            Self::Rparen => self.debug_fmt(f, "rparen"),
            Self::Semicolon => self.debug_fmt(f, "semicolon"),
            Self::Slash => self.debug_fmt(f, "slash"),
            Self::True => self.debug_fmt(f, "true"),
        }
    }
}
