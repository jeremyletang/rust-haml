// The MIT License (MIT)
//
// Copyright (c) 2014 Jeremy Letang (letang.jeremy@gmail.com)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::io::Reader;
use std::vec::Vec;

pub struct InputReader {
    input: ~Reader,
    buffer: Vec<char>,
    eof: bool
}

impl InputReader {
    pub fn new(input: ~Reader) -> InputReader {
        InputReader {
            input: input,
            buffer: Vec::new(),
            eof: false
        }
    }

    pub fn get(&mut self) -> Option<char> {
        if self.buffer.len() > 0 {
            self.buffer.shift()
        } else if self.eof {
            None
        } else {
            match self.input.read_byte() {
                Ok(b)   => Some(b as char),
                Err(_)  => None
            }
        }
    }

    pub fn unget(&mut self, c: char) {
        self.buffer.unshift(c)
    }

    pub fn unget_eof(&mut self) {
        self.eof = true
    }
}