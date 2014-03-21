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
    priv c_line: u32,
    priv indent_length: u32,
    priv indent_char: char,
    priv c_indent_lvl: u32
}

impl Parser {
    pub fn new(html_fmt: HtmlFormat) -> Parser {
        Parser {
            html_fmt: html_fmt,
            tokens: Vec::new(),
            dom_tree: DomTree::new(),
            c_line: 1,
            indent_length: 0,
            indent_char: 0u8 as char,
            c_indent_lvl: 0
        }
    }

    fn check_indent_on_first_line(&mut self,) -> Result<(), ~str> {
        loop {
            match self.tokens.get(0) {
                &token::EOL          => {
                    self.c_line += 1;
                    self.tokens.shift();
                },
                &token::INDENT(_, _) => return Err(error::illegal_indent_at_begin(self.c_line)),
                _                    => return Ok(())
            }
        }
    }

    fn mix_space_tab_indent(&mut self) -> Result<(), ~str> {
        match self.tokens.get(1) {
            &token::INDENT(_, _) => Err(error::indent_using_line_and_space(self.c_line)),
            _                    => Ok(())
        }
    }

    fn inconsistent_indent(&mut self, c: char, length: u32) -> Result<(), ~str> {
        if self.indent_char != c {
            Err(error::inconsistent_indent(self.c_line, c, self.indent_char,
                                           length, self.indent_length))
        } else if (length % self.indent_length) != 0 {
            Err(error::inconsistent_indent(self.c_line, c, c, length, self.indent_length))
        } else {
            Ok(())
        }
    }

    fn indent_level(&mut self, length: u32) -> Result<(), ~str> {
        let new_indent_lvl = length / self.indent_length;
        if new_indent_lvl == self.c_indent_lvl ||
           new_indent_lvl == self.c_indent_lvl + 1 ||
           new_indent_lvl < self.c_indent_lvl {
            self.c_indent_lvl = new_indent_lvl;
            Ok(())
        } else {
            Err(error::indent_level_much_deeper(self.c_line, new_indent_lvl - self.c_indent_lvl))
        }
    }

    fn check_indent(&mut self) -> Result<(), ~str> {
        match self.tokens.get(0) {
            &token::INDENT(c, l) => {
                if self.indent_length == 0 {
                    self.indent_length = l;
                    self.indent_char = c;
                }
                try!(self.mix_space_tab_indent());
                try!(self.inconsistent_indent(c, l));
                try!(self.indent_level(l));
                Ok(())
            },
            _                   => Ok(())
        }
    }

    pub fn execute(&mut self, tokens: Vec<Token>) -> Result<DomTree, ~str> {
        self.tokens = tokens;
        try!(self.check_indent_on_first_line());
        while self.tokens.get(0) != &token::EOF {
            try!(self.check_indent());
            if self.tokens.get(0) == &token::EOL {
                self.c_line += 1;
            }
            self.tokens.shift();
        }

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
        let tokens = vec!(token::INDENT(' ', 2), token::TAG(~"tag"), token::EOL, token::EOF);
        assert_err!(parser.execute(tokens))
    }

    #[test]
    fn document_beginning_with_no_indent_is_valid() {
        let mut parser = Parser::new(Html5);
        let tokens = vec!(token::TAG(~"tag"), token::EOL, token::EOF);
       assert_ok!(parser.execute(tokens))
    }

    #[test]
    fn document_beginning_with_eol_then_indent_is_invalid() {
        let mut parser = Parser::new(Html5);
        let tokens = vec!(token::EOL, token::INDENT(' ', 2), token::TAG(~"tag"), token::EOL,
                          token::EOF);
        assert_err!(parser.execute(tokens))
    }

    #[test]
    fn document_beginning_with_eol_then_no_indent_is_valid() {
        let mut parser = Parser::new(Html5);
        let tokens = vec!(token::EOL, token::TAG(~"tag"), token::EOL, token::EOF);
       assert_ok!(parser.execute(tokens))
    }

    #[test]
    fn cannot_indent_using_space_and_tabs_in_the_same_line() {
        let mut parser = Parser::new(Html5);
        let tokens = vec!(token::TAG(~"tag"), token::EOL,
                          token::INDENT(' ', 2), token::INDENT('\t', 2), token::EOL, token::EOF);
       assert_err!(parser.execute(tokens))
    }

    #[test]
    fn can_indent_document() {
        let mut parser = Parser::new(Html5);
        let tokens = vec!(token::TAG(~"tag"), token::EOL,
                          token::INDENT(' ', 2), token::TAG(~"tag"), token::EOL,
                          token::INDENT(' ', 4), token::TAG(~"tag"), token::EOL,
                          token::TAG(~"tag"), token::EOL,
                          token::EOF);
       assert_ok!(parser.execute(tokens))
    }

    #[test]
    fn can_indent_using_space_and_tabs_in_different_lines() {
        let mut parser = Parser::new(Html5);
        let tokens = vec!(token::TAG(~"tag"), token::EOL,
                          token::INDENT(' ', 2), token::TAG(~"tag"), token::EOL,
                          token::INDENT('\t', 4), token::TAG(~"tag"), token::EOL,
                          token::EOF);
       assert_err!(parser.execute(tokens))
    }

    #[test]
    fn can_indent_in_the_same_lvl() {
        let mut parser = Parser::new(Html5);
        let tokens = vec!(token::TAG(~"tag"), token::EOL,
                          token::INDENT(' ', 2), token::TAG(~"tag"), token::EOL,
                          token::INDENT(' ', 2), token::TAG(~"tag"), token::EOL,
                          token::INDENT(' ', 2), token::TAG(~"tag"), token::EOL,
                          token::INDENT(' ', 2), token::TAG(~"tag"), token::EOL,
                          token::EOF);
       assert_ok!(parser.execute(tokens))
    }

    #[test]
    fn cannot_omit_a_lvl() {
        let mut parser = Parser::new(Html5);
        let tokens = vec!(token::TAG(~"tag"), token::EOL,
                          token::INDENT(' ', 2), token::TAG(~"tag"), token::EOL,
                          token::INDENT(' ', 6), token::TAG(~"tag"), token::EOL,
                          token::EOF);
       assert_err!(parser.execute(tokens))
    }

    #[test]
    fn each_indent_have_the_same_base_length() {
        let mut parser = Parser::new(Html5);
        let tokens = vec!(token::TAG(~"tag"), token::EOL,
                          token::INDENT(' ', 2), token::TAG(~"tag"), token::EOL,
                          token::INDENT(' ', 4), token::TAG(~"tag"), token::EOL,
                          token::INDENT(' ', 6), token::TAG(~"tag"), token::EOL,
                          token::EOF);
       assert_ok!(parser.execute(tokens))
    }

    #[test]
    fn indent_cannot_have_different_base_length() {
        let mut parser = Parser::new(Html5);
        let tokens = vec!(token::TAG(~"tag"), token::EOL,
                          token::INDENT(' ', 2), token::TAG(~"tag"), token::EOL,
                          token::INDENT(' ', 3), token::TAG(~"tag"), token::EOL,
                          token::EOF);
       assert_err!(parser.execute(tokens))
    }
}
