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

#![crate_id = "haml#0.0.1"]
#![desc = "haml templating library for Rust"]
#![license = "MIT"]
#![crate_type = "bin"]
#![allow(missing_doc)]
#![allow(dead_code)]
#![allow(deprecated_owned_vector)]

#![feature(globs)]

extern crate haml;
extern crate colorize;

use std::io;
use std::io::File;
use std::path::Path;
use std::os;

use colorize::*;

fn get_reader() -> Result<~Reader, ~str> {
    let a = os::args();
    if a.len() == 1 {
        Ok(~io::stdin() as ~Reader)
    } else if a.len() == 2 {
        match File::open(&Path::new(a[1].clone())) {
            Ok(f)   => Ok(~f as ~Reader),
            Err(_)  => Err(format!("{} {}, no such file or directory.", "error:".b_red(), a[1]))
        }
    } else {
        Err(format!("{} invalid arguments number: expected 1 but found {}.", "error:".b_red(),
                    a.len()))
    }
}

fn print_usage() {
    println!("{} ./haml [optional: filepath]", "usage:".b_yellow())
}

fn main() {
    match get_reader() {
        Ok(reader)   => {
            // parse haml
            let mut haml_engine = haml::Engine::new(reader, haml::Html5);
            match haml_engine.execute() {
                Ok(_)  => { /* nothing to do */ }
                Err(e) => { println!("{}{}", "syntax error: ".b_red(), e); return }
            }
            // generate and write html.
            let mut writer = io::stdout();
            match haml_engine.generate(&mut writer as &mut Writer) {
                Ok(()) => { /* nothing to do */ },
                Err(e) => fail!("error on writing output: {}", e)
            }
        }
        Err(s)  => {
            println!("{}", s);
            print_usage();
        }
    }
}