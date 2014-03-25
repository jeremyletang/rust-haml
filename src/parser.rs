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
use collections::HashMap;

use dom_tree::{DomTree, Item};
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

pub struct DCollector {
    attributes: HashMap<~str, Vec<~str>>,
    tag: ~str,
    content: ~str,
}

impl DCollector {
    pub fn new() -> DCollector {
        DCollector {
            attributes: HashMap::new(),
            tag: ~"",
            content: ~""
        }
    }
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
                self.tokens.shift();
                Ok(())
            },
            _                   => Ok(())
        }
    }

    fn check_attributes(&mut self) -> Result<(), ~str> {
        Ok(())
    }

    fn check_tag(&mut self, data: &mut DCollector) -> Result<(), ~str> {
        fn invalid_id_class(name: &~str, line: u32) -> Result<(), ~str> {
            if name.len() == 0 {
                Err(error::illegal_element_class_id(line))
            } else {
                Ok(())
            }
        }
        match self.tokens.get(0) {
            &token::TAG(ref name)   => {
                if name.len() != 0 {
                    data.tag = name.to_owned();
                } else {
                    return Err(error::invalid_tag(self.c_line, ~"%"))
                }
            },
            &token::ID(ref name)    => {
                try!(invalid_id_class(name, self.c_line));
                data.attributes.insert(~"id", vec!(name.to_owned()));
            },
            &token::CLASS(ref name) => {
                try!(invalid_id_class(name, self.c_line));
                data.attributes.insert_or_update_with(~"class", vec!(name.to_owned()), |_, v| {
                    v.push(name.to_owned());
                });
            },
            _ => {}
        }
        try!(self.check_attributes());
        Ok(())
    }

    pub fn check_illegal_nesting(&self, data: &DCollector) -> Result<(), ~str> {
        match self.tokens.get(0) {
            &token::INDENT(_, l) => {
                if data.content != ~"" {
                    if l > self.indent_length && (data.tag != ~"" || !data.attributes.is_empty()) {
                        Err(error::illegal_nesting(self.c_line, data.tag.to_owned()))
                    } else if l > self.indent_length &&
                              (data.tag == ~"" && data.attributes.is_empty()) {
                        Err(error::illegal_plain_text_nesting(self.c_line))
                    } else {
                        Ok(())
                    }
                } else {
                    Ok(())
                }
            },
            _                    => Ok(())
        }
    }

    pub fn execute(&mut self, tokens: Vec<Token>) -> Result<DomTree, ~str> {
        self.tokens = tokens;
        try!(self.check_indent_on_first_line());
        let mut data: DCollector = DCollector::new();
        while self.tokens.get(0) != &token::EOF {
            match self.tokens.get(0).clone() {
                token::INDENT(_, _)      => try!(self.check_indent()),
                token::TAG(_)
                | token::ID(_)
                | token::CLASS(_)        => {
                    try!(self.check_tag(&mut data));
                    self.tokens.shift();
                },
                token::PLAIN_TEXT(ref s) => { data.content = s.clone(); self.tokens.shift(); },
                token::EOL               => {
                    self.tokens.shift();
                    try!(self.check_illegal_nesting(&data));
                    self.c_line += 1;
                    data = DCollector::new();
                },
                _                        => { self.tokens.shift(); }
            }
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

    #[test]
    fn tag_can_have_alphanumeric_char_in_name() {
        let mut parser = Parser::new(Html5);
        let tokens = vec!(token::TAG(~"tag"), token::EOL,
                          token::EOF);
       assert_ok!(parser.execute(tokens))
    }

    #[test]
    fn tag_cannot_be_empty_or_have_invalid_char() {
        let mut parser = Parser::new(Html5);
        let tokens = vec!(token::TAG(~""), token::EOL,
                          token::EOF);
       assert_err!(parser.execute(tokens))
    }

    #[test]
    fn class_can_have_alphanumeric_name() {
        let mut parser = Parser::new(Html5);
        let tokens = vec!(token::CLASS(~"class"), token::EOL,
                          token::EOF);
       assert_ok!(parser.execute(tokens))
    }

    #[test]
    fn id_can_have_alphanumeric_name() {
        let mut parser = Parser::new(Html5);
        let tokens = vec!(token::CLASS(~"id"), token::EOL,
                          token::EOF);
       assert_ok!(parser.execute(tokens))
    }

    #[test]
    fn class_name_cannot_be_empty_or_have_invalid_char() {
        let mut parser = Parser::new(Html5);
        let tokens = vec!(token::CLASS(~""), token::EOL,
                          token::EOF);
       assert_err!(parser.execute(tokens))
    }

    #[test]
    fn id_name_cannot_be_empty_or_have_invalid_char() {
        let mut parser = Parser::new(Html5);
        let tokens = vec!(token::CLASS(~""), token::EOL,
                          token::EOF);
       assert_err!(parser.execute(tokens))
    }

    #[test]
    fn content_on_the_same_line_and_nested_is_illegal() {
        let mut parser = Parser::new(Html5);
        let tokens = vec!(token::TAG(~"tag"), token::PLAIN_TEXT(~"Hello world"), token::EOL,
                          token::INDENT(' ', 2), token::TAG(~"tag2"), token::EOL, token::EOF);
       assert_err!(parser.execute(tokens))
    }

    #[test]
    fn content_can_be_on_the_same_line_if_not_nested() {
        let mut parser = Parser::new(Html5);
        let tokens = vec!(token::TAG(~"tag"), token::PLAIN_TEXT(~"Hello world"), token::EOL,
                          token::TAG(~"tag2"), token::EOL, token::EOF);
       assert_ok!(parser.execute(tokens))
    }

    #[test]
    fn content_can_be_inline_inside_a_nested_tag() {
        let mut parser = Parser::new(Html5);
        let tokens = vec!(token::TAG(~"tag"), token::EOL,
                          token::INDENT(' ', 2), token::TAG(~"tag2"),
                          token::PLAIN_TEXT(~"Hello world"), token::EOL,
                          token::TAG(~"tag"), token::EOL, token::EOF);
       assert_ok!(parser.execute(tokens))
    }

    #[test]
    fn plain_text_cannot_be_nested_within_plain_text() {
        let mut parser = Parser::new(Html5);
        let tokens = vec!(token::PLAIN_TEXT(~"Hello world"), token::EOL,
                          token::INDENT(' ', 2), token::PLAIN_TEXT(~"Hello world"), token::EOL,
                          token::EOF);
       assert_err!(parser.execute(tokens))
    }

    #[test]
    fn div_cannot_be_nested_within_plain_text() {
        let mut parser = Parser::new(Html5);
        let tokens = vec!(token::PLAIN_TEXT(~"Hello world"), token::EOL,
                          token::INDENT(' ', 2), token::TAG(~"tag"), token::EOL,
                          token::EOF);
       assert_err!(parser.execute(tokens))
    }

    #[test]
    fn plain_text_can_be_followed_by_plain_text() {
        let mut parser = Parser::new(Html5);
        let tokens = vec!(token::PLAIN_TEXT(~"Hello world"), token::EOL,
                          token::PLAIN_TEXT(~"Hello world"), token::EOL,
                          token::EOF);
       assert_ok!(parser.execute(tokens))
    }

    #[test]
    fn plain_text_can_be_followed_by_div() {
        let mut parser = Parser::new(Html5);
        let tokens = vec!(token::PLAIN_TEXT(~"Hello world"), token::EOL,
                          token::TAG(~"tag"), token::EOL,
                          token::EOF);
       assert_ok!(parser.execute(tokens))
    }
}
