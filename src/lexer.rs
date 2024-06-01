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

use crate::token::{Token, TokenType};

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
            Some('(') => Token::new(TokenType::Lparen),
            Some(')') => Token::new(TokenType::Rparen),
            Some('+') => Token::new(TokenType::Plus),
            Some(',') => Token::new(TokenType::Comma),
            Some(';') => Token::new(TokenType::Semicolon),
            Some('=') => Token::new(TokenType::Assign),
            Some('{') => Token::new(TokenType::Lbrace),
            Some('}') => Token::new(TokenType::Rbrace),
            None => Token::new(TokenType::Eof),
            Some(ch) => {
                if ch.is_alphabetic() {
                    let ident = self.read_identifier();
                    return self.lookup_ident(ident);
                } else if ch.is_digit(10) {
                    let number = self.read_number();
                    return Token::new(TokenType::Int(number));
                } else {
                    Token::new(TokenType::Illegal(ch.into()))
                }
            }
        };

        self.read_char();
        token
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
            if ch.is_digit(10) {
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
            "let" => Token::new(TokenType::Let),
            "fn" => Token::new(TokenType::Function),
            _ => Token::new(TokenType::Ident(ident)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenType;

    struct TestNextToken<'a> {
        // Expected token
        exp_t: TokenType,
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
        "#;
        let tests = vec![
            TestNextToken { exp_t: TokenType::Let, exp_l: "let" },
            TestNextToken {
                exp_t: TokenType::Ident("five".into()),
                exp_l: "five",
            },
            TestNextToken { exp_t: TokenType::Assign, exp_l: "=" },
            TestNextToken { exp_t: TokenType::Int("5".into()), exp_l: "5" },
            TestNextToken { exp_t: TokenType::Semicolon, exp_l: ";" },
            TestNextToken { exp_t: TokenType::Let, exp_l: "let" },
            TestNextToken {
                exp_t: TokenType::Ident("ten".into()),
                exp_l: "ten",
            },
            TestNextToken { exp_t: TokenType::Assign, exp_l: "=" },
            TestNextToken { exp_t: TokenType::Int("10".into()), exp_l: "10" },
            TestNextToken { exp_t: TokenType::Semicolon, exp_l: ";" },
            TestNextToken { exp_t: TokenType::Let, exp_l: "let" },
            TestNextToken {
                exp_t: TokenType::Ident("add".into()),
                exp_l: "add",
            },
            TestNextToken { exp_t: TokenType::Assign, exp_l: "=" },
            TestNextToken { exp_t: TokenType::Function, exp_l: "fn" },
            TestNextToken { exp_t: TokenType::Lparen, exp_l: "(" },
            TestNextToken { exp_t: TokenType::Ident("x".into()), exp_l: "x" },
            TestNextToken { exp_t: TokenType::Comma, exp_l: "," },
            TestNextToken { exp_t: TokenType::Ident("y".into()), exp_l: "y" },
            TestNextToken { exp_t: TokenType::Rparen, exp_l: ")" },
            TestNextToken { exp_t: TokenType::Lbrace, exp_l: "{" },
            TestNextToken { exp_t: TokenType::Ident("x".into()), exp_l: "x" },
            TestNextToken { exp_t: TokenType::Plus, exp_l: "+" },
            TestNextToken { exp_t: TokenType::Ident("y".into()), exp_l: "y" },
            TestNextToken { exp_t: TokenType::Semicolon, exp_l: ";" },
            TestNextToken { exp_t: TokenType::Rbrace, exp_l: "}" },
            TestNextToken { exp_t: TokenType::Semicolon, exp_l: ";" },
            TestNextToken { exp_t: TokenType::Let, exp_l: "let" },
            TestNextToken {
                exp_t: TokenType::Ident("result".into()),
                exp_l: "result",
            },
            TestNextToken { exp_t: TokenType::Assign, exp_l: "=" },
            TestNextToken {
                exp_t: TokenType::Ident("add".into()),
                exp_l: "add",
            },
            TestNextToken { exp_t: TokenType::Lparen, exp_l: "(" },
            TestNextToken {
                exp_t: TokenType::Ident("five".into()),
                exp_l: "five",
            },
            TestNextToken { exp_t: TokenType::Comma, exp_l: "," },
            TestNextToken {
                exp_t: TokenType::Ident("ten".into()),
                exp_l: "ten",
            },
            TestNextToken { exp_t: TokenType::Rparen, exp_l: ")" },
            TestNextToken { exp_t: TokenType::Semicolon, exp_l: ";" },
            TestNextToken { exp_t: TokenType::Eof, exp_l: "" },
        ];

        let mut lexer = Lexer::new(input);

        for (i, tt) in tests.iter().enumerate() {
            let token = lexer.next_token();
            assert_eq!(
                token.tp(),
                &tt.exp_t,
                "tests[{}] - token type wrong. expected={}, got={}",
                i,
                tt.exp_t,
                token.tp()
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
