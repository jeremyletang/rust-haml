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

use std::vec_ng::Vec;

use input_reader::InputReader;
use token::Token;
use token;

pub struct Lexer {
    priv input: InputReader,
    priv tokens: Vec<Token>
}

pub enum LexResult {
    Ok,
    Err(~str),
    End
}

impl Lexer {
    pub fn new(input: InputReader) -> Lexer {
        Lexer {
            input: input,
            tokens: Vec::new()
        }
    }

    pub fn execute(&mut self) {
        loop {
            match self.lex_line() {
                Ok          => {/* continue */},
                Err(msg)    => { println!("error: {}", msg); break },
                End         => break
           }
        }
        println!("tokens:\n{}", self.tokens);
    }

    fn get_all(&mut self, c: char) -> bool {
        let mut len = 0;
        loop {
            match self.input.get() {
                Some(next_c) => {
                    if next_c == c {
                        len += 1;
                    } else {
                        self.input.unget(Some(next_c));
                        break
                    }
                },
                None => { self.input.unget(None); break }
            }
        }
        if len > 0 {
            self.tokens.push(token::INDENT(c, len));
            true
        } else { false }
    }

    fn handle_indent(&mut self) {
        while self.get_all(' ') ||
            self.get_all('\t') {}
    }

    fn handle_plain_text(&mut self) {
        let mut content = ~"";
        loop {
            match self.input.get() {
                Some('\n')  => { self.input.unget(Some('\n')); break },
                Some(c)     => content.push_char(c),
                None        => { self.input.unget(None); break }
            }
        }
        if content.len() > 0 {
            self.tokens.push(token::PLAIN_TEXT(content));
        }
    }

    fn handle_comments(&mut self) -> bool {
        match self.input.get() {
            Some('-')    => {
                match self.input.get() {
                    Some('#')    => {
                        self.tokens.push(token::HAML_COMMENT);
                        self.handle_plain_text();
                        true
                    }
                    Some(next_c) => {
                        self.input.unget(Some(next_c));
                        self.input.unget(Some('-'));
                        false
                    },
                    None         => {
                        self.input.unget(None);
                        self.input.unget(Some('-'));
                        false
                    }
                }
            }
            Some('/')    => {
                self.tokens.push(token::HTML_COMMENT);
                self.handle_plain_text();
                true
            }
            Some(next_c) => { self.input.unget(Some(next_c)); false },
            None         => { self.input.unget(None); false }
        }
    }

    fn handle_tag(&mut self) {

    }

    fn handle_escape_line(&mut self) {
        match self.input.get() {
            Some('\\')   => {
                self.input.unget(Some('\\'));
                self.handle_plain_text();
            },
            Some(next_c) => self.input.unget(Some(next_c)),
            None         => self.input.unget(None)
        }
    }

    fn lex_line(&mut self) -> LexResult {
        self.handle_indent();
        self.handle_escape_line();
        // no comments found -> try to find a tag
        if !self.handle_comments() {
            self.handle_tag();
        }
        self.handle_plain_text();
        match self.input.get() {
            Some(_) => { self.tokens.push(token::EOL); Ok },
            None    => { self.tokens.push(token::EOF); End }
        }
    }
}
