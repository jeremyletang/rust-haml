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

use std::vec::Vec;

use dom_tree::{DomTree, DomElement};
use format::HtmlFormat;
use token::Token;
use token;
use error;

pub struct Parser {
    priv html_fmt: HtmlFormat,
    priv tokens: Vec<Token>,
    priv dom_tree: DomTree,
    priv c_line: uint,
    priv indent_length: u32,
    priv indent_char: char
}

impl Parser {
    pub fn new(html_fmt: HtmlFormat) -> Parser {
        Parser {
            html_fmt: html_fmt,
            tokens: Vec::new(),
            dom_tree: DomTree::new(),
            c_line: 1,
            indent_length: 0,
            indent_char: 0u8 as char
        }
    }

    fn check_indent_on_first_line(&mut self,) -> Result<(), ~str> {
        loop {
            match self.tokens.get(0) {
                &token::EOL          => {
                    self.c_line += 1;
                    self.tokens.shift();
                },
                &token::INDENT(_, _) =>
                    return Err(error::illegal_indent_at_begin(self.c_line)),
                _                    => return Ok(())
            }
        }
    }

    pub fn execute(&mut self, tokens: Vec<Token>) -> Result<DomTree, ~str> {
        self.tokens = tokens;
        try!(self.check_indent_on_first_line());

        Ok(DomTree::new())
    }
}

#[cfg(test)]
mod test {
    use token;
    use format::Html5;
    use parser::Parser;

    #[test]
    fn document_beginning_with_indent_is_invalid() {
        let mut parser = Parser::new(Html5);
        let tokens = vec!(token::INDENT(' ', 2), token::TAG(~"tag"),
                          token::EOL, token::EOF);
        assert_err!(parser.execute(tokens))
    }

    #[test]
    fn document_beginning_with_no_indent_is_valid() {
        let mut parser = Parser::new(Html5);
        let tokens = vec!(token::TAG(~"tag"),
                          token::EOL, token::EOF);
       assert_ok!(parser.execute(tokens))
    }

    #[test]
    fn document_beginning_with_eol_then_indent_is_invalid() {
        let mut parser = Parser::new(Html5);
        let tokens = vec!(token::EOL, token::INDENT(' ', 2),
                          token::TAG(~"tag"), token::EOL, token::EOF);
        assert_err!(parser.execute(tokens))
    }

    #[test]
    fn document_beginning_with_eol_then_no_indent_is_valid() {
        let mut parser = Parser::new(Html5);
        let tokens = vec!(token::EOL, token::TAG(~"tag"),
                          token::EOL, token::EOF);
       assert_ok!(parser.execute(tokens))
    }

}
