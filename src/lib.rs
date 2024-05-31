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

use std::io::{prelude::*, BufReader, BufWriter, Read, Write};

mod token;
mod lexer;

const PROMPT: &[u8] = b"[anlang]>> ";

pub fn start<R: Read, W: Write>(r: R, w: W) {
    let mut reader = BufReader::new(r);
    let mut writer = BufWriter::new(w);
    let mut line = String::new();

    loop {
        writer.write_all(PROMPT).unwrap();
        writer.flush().unwrap();
        let _ = reader.read_line(&mut line).unwrap();
        line.clear();
    }
}
