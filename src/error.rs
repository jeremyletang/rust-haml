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

pub fn illegal_indent_at_begin(line: u32) -> ~str {
    format!("line {}, indenting is forbidden at the beginning of the document.", line)
}

pub fn indent_using_line_and_space(line: u32) -> ~str {
    format!("line {}, indentation can't use both tabs and spaces.", line)
}

pub fn indent_level_much_deeper(line: u32, deep_indent: u32) -> ~str {
    format!("line {}, was indented {} levels deeper than the previous line.", line, deep_indent)
}

pub fn inconsistent_indent(line: u32, bad_c: char, c: char, bad_indent: u32, indent: u32) -> ~str {
    let name = if c == ' ' { ~"spaces" } else { ~"tabs" };
    let bad_name = if bad_c == ' ' { ~"spaces" } else { ~"tabs" };
    format!("line {}, inconsistent indentation: {} {} used for indentation, \
            but the rest of the document was indented using {} {}.",
            line,
            bad_indent,
            bad_name,
            indent,
            name )
}

pub fn invalid_tag(line: u32, name: ~str) -> ~str {
    format!("line {}, invalid tag name \"{}\"", line, name)
}

pub fn illegal_element_class_id(line: u32) -> ~str {
    format!("line {}, illegal element: classes and ids must have values.", line)
}

pub fn illegal_nesting(line: u32, tag_name: ~str) -> ~str {
    format!("line {}, illegal nesting: content can't be both given on the same line as \
             %{} and nested within it.", line, tag_name)
}

pub fn illegal_plain_text_nesting(line: u32) -> ~str {
    format!("line {}, illegal nesting: nesting within plain text is illegal", line)
}