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

#[allow(deprecated_owned_vector)];

pub static XHTML_1_0_Transitional: &'static str =
"<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \
\"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">";

pub static XHTML_1_0_Strict: &'static str =
"<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Strict//EN\" \
\"http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd\">";

pub static XHTML_1_0_Frameset: &'static str =
"<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Frameset//EN\" \
\"http://www.w3.org/TR/xhtml1/DTD/xhtml1-frameset.dtd\">";

pub static XHTML_5: &'static str = "<!DOCTYPE html>";

pub static XHTML_1_1: &'static str =
"<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.1//EN\" \
\"http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd\">";

pub static XHTML_Basic_1_1: &'static str =
"<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML Basic 1.1//EN\" \
\"http://www.w3.org/TR/xhtml-basic/xhtml-basic11.dtd\">";

pub static XHTML_Mobile_1_2: &'static str =
"<!DOCTYPE html PUBLIC \"-//WAPFORUM//DTD XHTML Mobile 1.2//EN\" \
\"http://www.openmobilealliance.org/tech/DTD/xhtml-mobile12.dtd\">";

pub static XHTML_PLUS_RDFA_1_0: &'static str =
"<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML+RDFa 1.0//EN\" \
\"http://www.w3.org/MarkUp/DTD/xhtml-rdfa-1.dtd\">";

pub static HTML_4_01_Transitional: &'static str =
"<!DOCTYPE html PUBLIC \"-//W3C//DTD HTML 4.01 Transitional//EN\" \
\"http://www.w3.org/TR/html4/loose.dtd\">";

pub static HTML_4_01_Strict: &'static str =
"<!DOCTYPE html PUBLIC \"-//W3C//DTD HTML 4.01//EN\" \
\"http://www.w3.org/TR/html4/strict.dtd\">";

pub static HTML_4_01_Frameset: &'static str =
"<!DOCTYPE html PUBLIC \"-//W3C//DTD HTML 4.01 Frameset//EN\" \
\"http://www.w3.org/TR/html4/frameset.dtd\">";

pub enum HtmlFormat {
    Xhtml,
    Html4,
    Html5
}

pub fn get_specific_format(spec_format: ~str, html_fmt: HtmlFormat) -> ~str {
    match xml_prolog(spec_format.clone()) {
        Some(s) => s,
        None    => {
            match html_fmt {
                Xhtml => get_xhtml_specific(spec_format),
                Html4 => get_html4_specific(spec_format),
                Html5 => get_html5_specific(spec_format)
            }
        }
    }
}

fn xml_prolog(spec_format: ~str) -> Option<~str> {
    let split: ~[&str] = spec_format.split(' ').collect();
    if split.len() > 0 {
        match split[0] {
            "XML"  => {
                if split.len() == 1 {
                    Some(~"<?xml version='1.0' encoding='utf-8' ?>")
                } else if split.len() == 2 {
                    Some(format!("<?xml version='1.0' encoding='{}' ?>",
                                 split[1]))
                } else {
                    None
                }
            },
            _      => None
        }
    } else {
        None
    }
}

fn get_xhtml_specific(spec_format: ~str) -> ~str {
    match spec_format.as_slice() {
        "Strict"   => XHTML_1_0_Strict.to_owned(),
        "Frameset" => XHTML_1_0_Frameset.to_owned(),
        "5"        => XHTML_5.to_owned(),
        "1.1"      => XHTML_1_1.to_owned(),
        "Basic"    => XHTML_Basic_1_1.to_owned(),
        "Mobile"   => XHTML_Mobile_1_2.to_owned(),
        "RDFa"     => XHTML_PLUS_RDFA_1_0.to_owned(),
        _          => XHTML_1_0_Transitional.to_owned()
    }
}

fn get_html4_specific(spec_format: ~str) -> ~str {
    match spec_format.as_slice() {
        "Strict"   => HTML_4_01_Strict.to_owned(),
        "Frameset" => HTML_4_01_Frameset.to_owned(),
        _          => HTML_4_01_Transitional.to_owned()
    }
}

fn get_html5_specific(_: ~str) -> ~str {
    XHTML_5.to_owned()
}

