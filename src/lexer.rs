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

use std::io::{Cursor, Read};

use crate::token::Token;

#[derive(Debug)]
pub(crate) struct Lexer<'a> {
    /// Source code input
    input: Cursor<&'a [u8]>,
    /// Current char under examination
    ch: Option<char>,
}

impl<'a> Lexer<'a> {
    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        while let Some(ch) = self.ch {
            if ch.is_alphabetic() {
                ident.push(ch);
                self.read_char();
            } else {
                break;
            }
        }
        ident
    }

    /// Return the next token
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            Some('(') => Token::Lparen,
            Some(')') => Token::Rparen,
            Some('+') => Token::Plus,
            Some(',') => Token::Comma,
            Some(';') => Token::Semicolon,
            Some('=') => {
                if let Some(ch) = self.peek_char() {
                    if ch == '=' {
                        self.read_char();
                        Token::Eq
                    } else {
                        Token::Assign
                    }
                } else {
                    Token::Assign
                }
            }
            Some('{') => Token::Lbrace,
            Some('}') => Token::Rbrace,
            Some('-') => Token::Minus,
            Some('!') => {
                if let Some(ch) = self.peek_char() {
                    if ch == '=' {
                        self.read_char();
                        Token::NotEq
                    } else {
                        Token::Bang
                    }
                } else {
                    Token::Bang
                }
            }
            Some('*') => Token::Asterisk,
            Some('/') => Token::Slash,
            Some('<') => Token::Lt,
            Some('>') => Token::Gt,
            None => Token::Eof,
            Some(ch) => {
                if ch.is_alphabetic() {
                    let ident = self.read_identifier();
                    return self.lookup_ident(ident);
                } else if ch.is_ascii_digit() {
                    let number = self.read_number();
                    return Token::Int(number);
                } else {
                    Token::Illegal(ch.into())
                }
            }
        };

        self.read_char();
        token
    }

    fn peek_char(&mut self) -> Option<char> {
        let current_position = self.input.position();
        let mut buffer = [0; 1];
        let n = self.input.read(&mut buffer).unwrap_or(0);
        self.input.set_position(current_position);
        if n == 0 {
            None
        } else {
            Some(buffer[0] as char)
        }
    }

    /// Create a new lexer with the given input
    pub fn new(input: &'a str) -> Self {
        let mut lexer =
            Lexer { input: Cursor::new(input.as_bytes()), ch: None };
        lexer.read_char();
        lexer
    }

    /// Read the next character and advance the positions
    fn read_char(&mut self) {
        let mut buffer = [0; 1];
        let n = self.input.read(&mut buffer).unwrap_or(0);
        self.ch = if n == 0 { None } else { Some(buffer[0] as char) }
    }

    fn read_number(&mut self) -> String {
        let mut number = String::new();
        while let Some(ch) = self.ch {
            if ch.is_ascii_digit() {
                number.push(ch);
                self.read_char();
            } else {
                break;
            }
        }
        number
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.ch {
            if ch.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn lookup_ident(&self, ident: String) -> Token {
        match ident.as_str() {
            "let" => Token::Let,
            "fn" => Token::Function,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            _ => Token::Ident(ident),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestNextToken<'a> {
        // Expected token
        exp_t: Token,
        // Expected literal
        exp_l: &'a str,
    }

    #[test]
    fn next_token() {
        let input = r#"let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 5;
        "#;
        let tests = vec![
            TestNextToken { exp_t: Token::Let, exp_l: "let" },
            TestNextToken { exp_t: Token::Ident("five".into()), exp_l: "five" },
            TestNextToken { exp_t: Token::Assign, exp_l: "=" },
            TestNextToken { exp_t: Token::Int("5".into()), exp_l: "5" },
            TestNextToken { exp_t: Token::Semicolon, exp_l: ";" },
            TestNextToken { exp_t: Token::Let, exp_l: "let" },
            TestNextToken { exp_t: Token::Ident("ten".into()), exp_l: "ten" },
            TestNextToken { exp_t: Token::Assign, exp_l: "=" },
            TestNextToken { exp_t: Token::Int("10".into()), exp_l: "10" },
            TestNextToken { exp_t: Token::Semicolon, exp_l: ";" },
            TestNextToken { exp_t: Token::Let, exp_l: "let" },
            TestNextToken { exp_t: Token::Ident("add".into()), exp_l: "add" },
            TestNextToken { exp_t: Token::Assign, exp_l: "=" },
            TestNextToken { exp_t: Token::Function, exp_l: "fn" },
            TestNextToken { exp_t: Token::Lparen, exp_l: "(" },
            TestNextToken { exp_t: Token::Ident("x".into()), exp_l: "x" },
            TestNextToken { exp_t: Token::Comma, exp_l: "," },
            TestNextToken { exp_t: Token::Ident("y".into()), exp_l: "y" },
            TestNextToken { exp_t: Token::Rparen, exp_l: ")" },
            TestNextToken { exp_t: Token::Lbrace, exp_l: "{" },
            TestNextToken { exp_t: Token::Ident("x".into()), exp_l: "x" },
            TestNextToken { exp_t: Token::Plus, exp_l: "+" },
            TestNextToken { exp_t: Token::Ident("y".into()), exp_l: "y" },
            TestNextToken { exp_t: Token::Semicolon, exp_l: ";" },
            TestNextToken { exp_t: Token::Rbrace, exp_l: "}" },
            TestNextToken { exp_t: Token::Semicolon, exp_l: ";" },
            TestNextToken { exp_t: Token::Let, exp_l: "let" },
            TestNextToken {
                exp_t: Token::Ident("result".into()),
                exp_l: "result",
            },
            TestNextToken { exp_t: Token::Assign, exp_l: "=" },
            TestNextToken { exp_t: Token::Ident("add".into()), exp_l: "add" },
            TestNextToken { exp_t: Token::Lparen, exp_l: "(" },
            TestNextToken { exp_t: Token::Ident("five".into()), exp_l: "five" },
            TestNextToken { exp_t: Token::Comma, exp_l: "," },
            TestNextToken { exp_t: Token::Ident("ten".into()), exp_l: "ten" },
            TestNextToken { exp_t: Token::Rparen, exp_l: ")" },
            TestNextToken { exp_t: Token::Semicolon, exp_l: ";" },
            // operators
            TestNextToken { exp_t: Token::Bang, exp_l: "!" },
            TestNextToken { exp_t: Token::Minus, exp_l: "-" },
            TestNextToken { exp_t: Token::Slash, exp_l: "/" },
            TestNextToken { exp_t: Token::Asterisk, exp_l: "*" },
            TestNextToken { exp_t: Token::Int("5".into()), exp_l: "5" },
            TestNextToken { exp_t: Token::Semicolon, exp_l: ";" },
            TestNextToken { exp_t: Token::Int("5".into()), exp_l: "5" },
            TestNextToken { exp_t: Token::Lt, exp_l: "<" },
            TestNextToken { exp_t: Token::Int("10".into()), exp_l: "10" },
            TestNextToken { exp_t: Token::Gt, exp_l: ">" },
            TestNextToken { exp_t: Token::Int("5".into()), exp_l: "5" },
            TestNextToken { exp_t: Token::Semicolon, exp_l: ";" },
            // if ...
            TestNextToken { exp_t: Token::If, exp_l: "if" },
            TestNextToken { exp_t: Token::Lparen, exp_l: "(" },
            TestNextToken { exp_t: Token::Int("5".into()), exp_l: "5" },
            TestNextToken { exp_t: Token::Lt, exp_l: "<" },
            TestNextToken { exp_t: Token::Int("10".into()), exp_l: "10" },
            TestNextToken { exp_t: Token::Rparen, exp_l: ")" },
            TestNextToken { exp_t: Token::Lbrace, exp_l: "{" },
            TestNextToken { exp_t: Token::Return, exp_l: "return" },
            TestNextToken { exp_t: Token::True, exp_l: "true" },
            TestNextToken { exp_t: Token::Semicolon, exp_l: ";" },
            TestNextToken { exp_t: Token::Rbrace, exp_l: "}" },
            TestNextToken { exp_t: Token::Else, exp_l: "else" },
            TestNextToken { exp_t: Token::Lbrace, exp_l: "{" },
            TestNextToken { exp_t: Token::Return, exp_l: "return" },
            TestNextToken { exp_t: Token::False, exp_l: "false" },
            TestNextToken { exp_t: Token::Semicolon, exp_l: ";" },
            TestNextToken { exp_t: Token::Rbrace, exp_l: "}" },
            // statements
            TestNextToken { exp_t: Token::Int("10".into()), exp_l: "10" },
            TestNextToken { exp_t: Token::Eq, exp_l: "==" },
            TestNextToken { exp_t: Token::Int("10".into()), exp_l: "10" },
            TestNextToken { exp_t: Token::Semicolon, exp_l: ";" },
            TestNextToken { exp_t: Token::Int("10".into()), exp_l: "10" },
            TestNextToken { exp_t: Token::NotEq, exp_l: "!=" },
            TestNextToken { exp_t: Token::Int("5".into()), exp_l: "5" },
            TestNextToken { exp_t: Token::Semicolon, exp_l: ";" },
            // eof
            TestNextToken { exp_t: Token::Eof, exp_l: "" },
        ];

        let mut lexer = Lexer::new(input);

        for (i, tt) in tests.iter().enumerate() {
            let token = lexer.next_token();
            assert_eq!(
                token, tt.exp_t,
                "tests[{}] - token type wrong. expected={}, got={}",
                i, tt.exp_t, token
            );
            assert_eq!(
                token.literal(),
                tt.exp_l,
                "tests[{}] - literal wrong. expected={}, got={}",
                i,
                tt.exp_l,
                token.literal()
            );
        }
    }
}
