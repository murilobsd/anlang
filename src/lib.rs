#![allow(dead_code)]
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

use std::io::{self, prelude::*, BufReader, BufWriter, Read, Write};

mod lexer;
mod token;

use lexer::Lexer;
use token::TokenType;

const PROMPT: &[u8] = b">> ";

pub fn start<R: Read, W: Write>(r: R, w: W) -> io::Result<()> {
    let mut reader = BufReader::new(r);
    let mut writer = BufWriter::new(w);
    let mut line = String::new();

    loop {
        writer.write_all(PROMPT)?;
        writer.flush()?;
        let n = reader.read_line(&mut line)?;
        if n > 0 {
            let mut lex = Lexer::new(&line);
            loop {
                let token = lex.next_token();
                if token.tp() != &TokenType::Eof {
                    write!(writer, "{:?}\n", token)?;
                } else {
                    break;
                }
            }
            writer.flush()?;
        }
        line.clear();
    }
}
