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

#![allow(unused_variable)]

use std::fmt;
use std::io::Reader;
use std::vec::Vec;
use std::io::IoResult;

use format::{HtmlFormat, Xhtml, Html5, Html4};
use token::*;
use lexer::Lexer;
use parser::Parser;
use dom_tree::DomTree;
use input_reader::InputReader;

pub struct Engine {
    priv lexer: Lexer,
    priv parser: Parser,
    priv dom_tree: DomTree
}

impl Engine {
    pub fn new(input: ~Reader, html_fmt: HtmlFormat) -> Engine {
        Engine {
            lexer: Lexer::new(InputReader::new(input)),
            parser: Parser::new(html_fmt),
            dom_tree: DomTree::new()
        }
    }

    pub fn execute(&mut self) -> Result<(), ~str> {
        let tokens = self.lexer.execute();
        println!("tokens:\n{}", tokens);
        match self.parser.execute(tokens) {
            Ok(dt) => { self.dom_tree = dt; Ok(()) }
            Err(e) => Err(e)
        }
    }

    pub fn set_val<T: fmt::Show>(&mut self, val: T) -> bool {
        unimplemented!()
    }

    pub fn set_vec_val<T: fmt::Show>(&mut self, vec_val: Vec<T>) -> bool {
        unimplemented!()
    }

    pub fn generate(&mut self, output: &mut Writer) -> IoResult<()> {
        match output.write_str(format!("{}", self.dom_tree)) {
            Ok(_)   => Ok(()),
            Err(e)  => Err(e)
        }
    } 
}
