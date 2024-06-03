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
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.literal())
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Assign => f
                .debug_struct("Token")
                .field("type", &"assign")
                .field("literal", &self.literal())
                .finish(),
            Self::Asterisk => f
                .debug_struct("Token")
                .field("type", &"asterisk")
                .field("literal", &self.literal())
                .finish(),
            Self::Bang => f
                .debug_struct("Token")
                .field("type", &"bang")
                .field("literal", &self.literal())
                .finish(),
            Self::Comma => f
                .debug_struct("Token")
                .field("type", &"comma")
                .field("literal", &self.literal())
                .finish(),
            Self::Else => f
                .debug_struct("Token")
                .field("type", &"else")
                .field("literal", &self.literal())
                .finish(),
            Self::Eof => f
                .debug_struct("Token")
                .field("type", &"eof")
                .field("literal", &self.literal())
                .finish(),
            Self::Eq => f
                .debug_struct("Token")
                .field("type", &"eq")
                .field("literal", &self.literal())
                .finish(),
            Self::False => f
                .debug_struct("Token")
                .field("type", &"false")
                .field("literal", &self.literal())
                .finish(),
            Self::Function => f
                .debug_struct("Token")
                .field("type", &"function")
                .field("literal", &self.literal())
                .finish(),
            Self::Gt => f
                .debug_struct("Token")
                .field("type", &"gt")
                .field("literal", &self.literal())
                .finish(),
            Self::Ident(_) => f
                .debug_struct("Token")
                .field("type", &"ident")
                .field("literal", &self.literal())
                .finish(),
            Self::If => f
                .debug_struct("Token")
                .field("type", &"if")
                .field("literal", &self.literal())
                .finish(),
            Self::Illegal(_) => f
                .debug_struct("Token")
                .field("type", &"illegal")
                .field("literal", &self.literal())
                .finish(),
            Self::Int(_) => f
                .debug_struct("Token")
                .field("type", &"int")
                .field("literal", &self.literal())
                .finish(),
            Self::Lbrace => f
                .debug_struct("Token")
                .field("type", &"lbrace")
                .field("literal", &self.literal())
                .finish(),
            Self::Let => f
                .debug_struct("Token")
                .field("type", &"let")
                .field("literal", &self.literal())
                .finish(),
            Self::Lparen => f
                .debug_struct("Token")
                .field("type", &"lparen")
                .field("literal", &self.literal())
                .finish(),
            Self::Lt => f
                .debug_struct("Token")
                .field("type", &"lt")
                .field("literal", &self.literal())
                .finish(),
            Self::Minus => f
                .debug_struct("Token")
                .field("type", &"minus")
                .field("literal", &self.literal())
                .finish(),
            Self::NotEq => f
                .debug_struct("Token")
                .field("type", &"noteq")
                .field("literal", &self.literal())
                .finish(),
            Self::Plus => f
                .debug_struct("Token")
                .field("type", &"plus")
                .field("literal", &self.literal())
                .finish(),
            Self::Rbrace => f
                .debug_struct("Token")
                .field("type", &"rbrace")
                .field("literal", &self.literal())
                .finish(),
            Self::Return => f
                .debug_struct("Token")
                .field("type", &"return")
                .field("literal", &self.literal())
                .finish(),
            Self::Rparen => f
                .debug_struct("Token")
                .field("type", &"rparen")
                .field("literal", &self.literal())
                .finish(),
            Self::Semicolon => f
                .debug_struct("Token")
                .field("type", &"semicolon")
                .field("literal", &self.literal())
                .finish(),
            Self::Slash => f
                .debug_struct("Token")
                .field("type", &"slash")
                .field("literal", &self.literal())
                .finish(),
            Self::True => f
                .debug_struct("Token")
                .field("type", &"true")
                .field("literal", &self.literal())
                .finish(),
        }
    }
}
