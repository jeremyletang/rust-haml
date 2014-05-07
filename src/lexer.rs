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

use input_reader::InputReader;
use token::Token;
use token;

pub struct Lexer {
    input: InputReader,
    tokens: Vec<Token>
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

    pub fn execute(&mut self) -> Vec<Token> {
        loop {
            match self.lex_line() {
                Ok          => {/* continue */},
                Err(msg)    => { println!("error: {}", msg); break },
                End         => break
           }
        }
        self.tokens.clone()
    }

    fn next_is(&mut self, c: char) -> bool {
        match self.input.get() {
            Some(next_c) => {
                if next_c == c {
                    true
                } else {
                    self.input.unget(next_c);
                    false
                }
            },
            None         => {
                self.input.unget_eof();
                false
            }
        }
    }

    fn get_all(&mut self, c: char) -> bool {
        let mut len = 0;
        loop {
            match self.input.get() {
                Some(next_c) => {
                    if next_c == c {
                        len += 1;
                    } else {
                        self.input.unget(next_c);
                        break
                    }
                },
                None => { self.input.unget_eof(); break }
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
        fn clean_plain_text_after(c: Option<u8>, mut content: StrBuf) -> StrBuf {
            match c {
                Some(c) => {
                    if c as char == ' ' || c as char == '\t' {
                        clean_plain_text_after(unsafe {content.pop_byte() }, content)
                    } else { content.push_char(c as char); content }
                },
                _                      => content
            }
        }

        fn clean_plain_text_before(c: Option<u8>, mut content: StrBuf) -> StrBuf {
            match c {
                Some(c) => {
                    if c as char == ' ' || c as char == '\t' {
                        clean_plain_text_before(unsafe { content.shift_byte() }, content)
                    } else {
                        let mut tmp = StrBuf::from_char(1, c as char);
                        tmp.push_str(content.into_owned()); tmp }
                },
                _                      => content
            }
        }


        let mut content = StrBuf::new();
        loop {
            match self.input.get() {
                Some('\n')  => { self.input.unget('\n'); break },
                Some(c)     => content.push_char(c),
                None        => { self.input.unget_eof(); break }
            }
        }

        // remove whitespace before the text
        content = clean_plain_text_before(unsafe { content.shift_byte() }, content);
        // remove whitespace after the text
        content = clean_plain_text_after(unsafe { content.pop_byte() }, content);

        if content.len() > 0 { self.tokens.push(token::PLAIN_TEXT(content.into_owned()));  }
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
                        self.input.unget(next_c);
                        self.input.unget('-');
                        false
                    },
                    None         => {
                        self.input.unget_eof();
                        self.input.unget('-');
                        false
                    }
                }
            }
            Some('/')    => {
                self.tokens.push(token::HTML_COMMENT);
                self.handle_plain_text();
                true
            }
            Some(next_c) => { self.input.unget(next_c); false },
            None         => { self.input.unget_eof(); false }
        }
    }

    fn handle_doctype(&mut self) {
        if self.next_is('!') {
            if self.next_is('!') {
                self.tokens.push(token::DOCTYPE)
            } else {
                self.input.unget('!');
                self.input.unget('!');
            }
        } else {
            self.input.unget('!')
        }
    }

    fn handle_identifier(&mut self) -> ~str{
        let mut name = StrBuf::new();
        loop {
            match self.input.get() {
                Some(c) => {
                    if c.is_alphanumeric() || c == '-' || c == '_' {
                        name.push_char(c);
                    } else {
                        self.input.unget(c);
                        break
                    }
                }
                None    =>  { self.input.unget_eof(); break }
            }
        }
        name.into_owned()
    }

    fn handle_tag(&mut self) {
        // check first if there is a '%' tag
        if self.next_is('%') {
            let identifier = self.handle_identifier();
            self.tokens.push(token::TAG(identifier));
        }

        // then check for additionnal '.' class or '#' id
        loop {
            match self.input.get() {
                Some('!') => { self.handle_doctype(); break },
                Some('#') => {
                    let identifier = self.handle_identifier();
                    self.tokens.push(token::ID(identifier));
                },
                Some('.') => {
                    let identifier = self.handle_identifier();
                    self.tokens.push(token::CLASS(identifier));
                },
                Some(c_next) => { self.input.unget(c_next); break },
                None         => { self.input.unget_eof(); break }
            }
        }
    }

    fn handle_attribute(&mut self) {
        unimplemented!()
    }

    fn handle_escape_line(&mut self) {
        if self.next_is('\\') {
            self.handle_plain_text();
        }
    }

    fn handle_assign(&mut self) {
        if self.next_is('=') {
            self.tokens.push(token::ASSIGN)
        }
    }

    fn check_blankline(&mut self) {
        fn get_blankline_begin(v: &Vec<Token>, i: uint) -> uint {
            match v.get(i) {
                &token::INDENT(_, _) => if i == 0 { i } else { get_blankline_begin(v, i - 1) },
                _                   => i + 1
            }
        }

        let i = self.tokens.len();
        if i != 0 {
            match self.tokens.get(i - 1) {
                &token::INDENT(_, _) => {
                    let pos = get_blankline_begin(&self.tokens, i - 1);
                    self.tokens.truncate(pos);
                },
                _                   => { /* do nothing  */ }
            }
        }

    }

    fn handle_empty_tag(&mut self) {
        if self.next_is('/') {
            self.tokens.push(token::CLOSING_EMPTY);
        }
    }

    fn lex_line(&mut self) -> LexResult {
        self.handle_indent();
        self.handle_escape_line();
        // no comments found -> try to find a tag
        if !self.handle_comments() {
            self.handle_tag();
            // self.handle_attribute();
            self.handle_empty_tag();
            self.handle_assign();
            self.handle_plain_text();
        }
        match self.input.get() {
            Some(_) => {
                self.check_blankline();
                self.tokens.push(token::EOL);
                Ok
            },
            None    => { self.tokens.push(token::EOF); End }
        }
    }
}

#[cfg(test)]
mod tests {
    use lexer::Lexer;
    use input_reader::InputReader;
    use token;

    mod mock {
        use std::io::{Reader, IoError, EndOfFile, IoResult};

        pub struct Input {
            pub input: ~str
        }

        impl Reader for Input {
            fn read(&mut self, _: &mut [u8]) -> IoResult<uint> {
                Ok(0)
            }

            fn read_byte(&mut self) -> IoResult<u8> {
                match self.input.shift_char() {
                    Some(c) => Ok(c as u8),
                    None    => Err(IoError {
                        kind: EndOfFile,
                        desc: "",
                        detail: None,
                    })
                }
            }
        }
    }

    fn prepare_test_lexer(haml_str: ~str) -> Lexer {
        let input_reader = InputReader::new(~mock::Input { input: haml_str } as ~Reader);
        Lexer::new(input_reader)
    }

    #[test]
    fn lex_plain_text() {
        let haml_str = ~"this is a plain text string\n";
        let expected = vec!(token::PLAIN_TEXT(~"this is a plain text string"), token::EOL,
                            token::EOF);
        let mut lexer = prepare_test_lexer(haml_str);

        assert_eq!(expected, lexer.execute())
    }

    #[test]
    fn lex_empty_input() {
        let haml_str = ~"";
        let expected = vec!(token::EOF);
        let mut lexer = prepare_test_lexer(haml_str);

        assert_eq!(expected, lexer.execute())
    }

    #[test]
    fn lex_white_space_on_begin_give_indent() {
        let haml_str = ~"  %tag\n";
        let expected = vec!(token::INDENT(' ', 2), token::TAG(~"tag"), token::EOL, token::EOF);
        let mut lexer = prepare_test_lexer(haml_str);

        assert_eq!(expected, lexer.execute())
    }

    #[test]
    fn lex_a_line_ending_with_return() {
        let haml_str = ~"  %tag#id.class text\n";
        let expected = vec!(token::INDENT(' ', 2), token::TAG(~"tag"), token::ID(~"id"),
                            token::CLASS(~"class"), token::PLAIN_TEXT(~"text"), token::EOL,
                            token::EOF);
        let mut lexer = prepare_test_lexer(haml_str);

        assert_eq!(expected, lexer.execute())
    }

    #[test]
    fn lex_a_line_ending_with_end_of_file() {
        let haml_str = ~"  %tag#id.class text";
        let expected = vec!(token::INDENT(' ', 2), token::TAG(~"tag"), token::ID(~"id"),
                            token::CLASS(~"class"), token::PLAIN_TEXT(~"text"), token::EOF);
        let mut lexer = prepare_test_lexer(haml_str);

        assert_eq!(expected, lexer.execute())
    }

    #[test]
    fn lex_plain_text_and_remove_extra_white_space() {
        let haml_str = ~"%t \t   plain text   \t   \t   \t\n";
        let expected = vec!(token::TAG(~"t"), token::PLAIN_TEXT(~"plain text"), token::EOL,
                            token::EOF);
        let mut lexer = prepare_test_lexer(haml_str);

        assert_eq!(expected, lexer.execute())
    }

    #[test]
    fn lex_escape_string_give_plain_text() {
        let haml_str = ~"\\%t.i#4 + plain text string\n";
        let expected = vec!(token::PLAIN_TEXT(~"%t.i#4 + plain text string"), token::EOL,
                            token::EOF);
        let mut lexer = prepare_test_lexer(haml_str);

        assert_eq!(expected, lexer.execute())
    }

    #[test]
    fn lex_indent_with_escape_string_give_indent_and_plain_text() {
        let haml_str = ~"  \\%t.i#4 + plain text string\n";
        let expected = vec!(token::INDENT(' ', 2), token::PLAIN_TEXT(~"%t.i#4 + plain text string"),
                            token::EOL, token::EOF);
        let mut lexer = prepare_test_lexer(haml_str);

        assert_eq!(expected, lexer.execute())
    }

    #[test]
    fn lex_chained_tag_id_and_class() {
        let haml_str = ~"%tag.class#id.class\n";
        let expected = vec!(token::TAG(~"tag"), token::CLASS(~"class"), token::ID(~"id"),
                            token::CLASS(~"class"), token::EOL, token::EOF);
        let mut lexer = prepare_test_lexer(haml_str);

        assert_eq!(expected, lexer.execute())
    }

    #[test]
    fn remove_blankline_let_return() {
        let haml_str = ~"    \t     \n     \t    \n";
        let expected = vec!(token::EOL, token::EOL, token::EOF);
        let mut lexer = prepare_test_lexer(haml_str);

        assert_eq!(expected, lexer.execute())
    }

    #[test]
    fn remove_blankline_with_content() {
        let haml_str = ~"    \t     \n%t\n     \t    \n";
        let expected = vec!(token::EOL, token::TAG(~"t"), token::EOL, token::EOL, token::EOF);
        let mut lexer = prepare_test_lexer(haml_str);

        assert_eq!(expected, lexer.execute())
    }

    #[test]
    fn remove_blankline_inside_content() {
        let haml_str = ~"    \t     \n%t\n     \t    \n%i\n";
        let expected = vec!(token::EOL, token::TAG(~"t"), token::EOL, token::EOL, token::TAG(~"i"),
                            token::EOL, token::EOF);
        let mut lexer = prepare_test_lexer(haml_str);

        assert_eq!(expected, lexer.execute())
    }

    #[test]
    fn remove_blankline_and_keep_indent() {
        let haml_str = ~"    \t     \n  %t\n     \t    \n";
        let expected = vec!(token::EOL, token::INDENT(' ', 2), token::TAG(~"t"), token::EOL,
                            token::EOL, token::EOF);
        let mut lexer = prepare_test_lexer(haml_str);

        assert_eq!(expected, lexer.execute())
    }

    #[test]
    fn blank_space_after_tag_is_not_plain_text() {
        let haml_str = ~"%t          \n";
        let expected = vec!(token::TAG(~"t"), token::EOL, token::EOF);
        let mut lexer = prepare_test_lexer(haml_str);

        assert_eq!(expected, lexer.execute())
    }

    #[test]
    fn three_exclamation_and_more_is_doctype() {
        let haml_str = ~"!!!\n";
        let expected = vec!(token::DOCTYPE, token::EOL, token::EOF);
        let mut lexer = prepare_test_lexer(haml_str);

        assert_eq!(expected, lexer.execute())
    }

    #[test]
    fn doctype_can_have_plain_text_specifier() {
        let haml_str = ~"!!! Strict\n";
        let expected = vec!(token::DOCTYPE, token::PLAIN_TEXT(~"Strict"), token::EOL, token::EOF);
        let mut lexer = prepare_test_lexer(haml_str);

        assert_eq!(expected, lexer.execute())
    }

    #[test]
    fn one_exclamation_is_not_doctype() {
        let haml_str = ~"!!\n";
        let expected = vec!(token::PLAIN_TEXT(~"!!"), token::EOL, token::EOF);
        let mut lexer = prepare_test_lexer(haml_str);

        assert_eq!(expected, lexer.execute())
    }

    #[test]
    fn two_exclamation_is_not_doctype() {
        let haml_str = ~"!\n";
        let expected = vec!(token::PLAIN_TEXT(~"!"), token::EOL, token::EOF);
        let mut lexer = prepare_test_lexer(haml_str);

        assert_eq!(expected, lexer.execute())
    }

    #[test]
    fn lex_html_comment_with_no_new_line_give_plain_text() {
        let haml_str = ~"/ %t hello world";
        let expected = vec!(token::HTML_COMMENT, token::PLAIN_TEXT(~"%t hello world"), token::EOF);
        let mut lexer = prepare_test_lexer(haml_str);

        assert_eq!(expected, lexer.execute())
    }

    #[test]
    fn lex_html_comment_with_nested_tag_give_a_tag() {
        let haml_str = ~"/ \n  %t hello world";
        let expected = vec!(token::HTML_COMMENT, token::EOL, token::INDENT(' ', 2),
                            token::TAG(~"t"), token::PLAIN_TEXT(~"hello world"), token::EOF);
        let mut lexer = prepare_test_lexer(haml_str);

        assert_eq!(expected, lexer.execute())
    }

    #[test]
    fn lex_haml_comment_with_no_new_line_give_plain_text() {
        let haml_str = ~"-# %t hello world";
        let expected = vec!(token::HAML_COMMENT, token::PLAIN_TEXT(~"%t hello world"), token::EOF);
        let mut lexer = prepare_test_lexer(haml_str);

        assert_eq!(expected, lexer.execute())
    }

    #[test]
    fn lex_haml_comment_with_nested_tag_give_a_tag() {
        let haml_str = ~"-# \n  %t hello world";
        let expected = vec!(token::HAML_COMMENT, token::EOL, token::INDENT(' ', 2),
                            token::TAG(~"t"), token::PLAIN_TEXT(~"hello world"), token::EOF);
        let mut lexer = prepare_test_lexer(haml_str);

        assert_eq!(expected, lexer.execute())
    }

    #[test]
    fn lex_equal_on_new_ligne_give_assign() {
        let haml_str = ~"%t\n  =";
        let expected = vec!(token::TAG(~"t"), token::EOL, token::INDENT(' ', 2), token::ASSIGN,
                            token::EOF);
        let mut lexer = prepare_test_lexer(haml_str);

        assert_eq!(expected, lexer.execute())
    }

    #[test]
    fn lex_equal_stuck_top_tag_give_assign() {
        let haml_str = ~"%t=";
        let expected = vec!(token::TAG(~"t"), token::ASSIGN, token::EOF);
        let mut lexer = prepare_test_lexer(haml_str);

        assert_eq!(expected, lexer.execute())
    }

    #[test]
    fn lex_equal_not_stuck_to_tag_is_plain_text() {
        let haml_str = ~"%t =";
        let expected = vec!(token::TAG(~"t"), token::PLAIN_TEXT(~"="), token::EOF);
        let mut lexer = prepare_test_lexer(haml_str);

        assert_eq!(expected, lexer.execute())
    }

    #[test]
    fn find_slash_after_tag_create_empty_tag() {
        let haml_str = ~"%t/ hello";
        let expected = vec!(token::TAG(~"t"), token::CLOSING_EMPTY, token::PLAIN_TEXT(~"hello"),
                            token::EOF);
        let mut lexer = prepare_test_lexer(haml_str);

        assert_eq!(expected, lexer.execute())
    }
}
