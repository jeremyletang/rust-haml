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

#![macro_escape]

#[macro_export]
macro_rules! assert_err(
    ($arg:expr) => (
        match $arg {
            Ok(_)  => fail!("assertion failed: {:s} sould be Err", stringify!($arg)),
            Err(_) => {}
        }
    );
)

#[macro_export]
macro_rules! assert_ok(
    ($arg:expr) => (
        match $arg {
            Ok(_)  => {},
            Err(e) => fail!("assertion failed: {:s} sould be Ok, err: {}", stringify!($arg), e)
        }
    );
)

#[macro_export]
macro_rules! assert_some(
    ($arg:expr) => (
        match $arg {
            Some(_) => fail!("assertion failed: {:s} sould be None", stringify!($arg)),
            None    => {}
        }
    );
)

#[macro_export]
macro_rules! assert_none(
    ($arg:expr) => (
        match $arg {
            Some(_) => {},
            None    => fail!("assertion failed: {:s} sould be Some", stringify!($arg))
        }
    );
)

#[macro_export]
macro_rules! assert_true(
    ($arg:expr) => (
        match $arg {
            true  => {},
            false => fail!("assertion failed: {:s} sould be true", stringify!($arg))
        }
    );
)

#[macro_export]
macro_rules! assert_false(
    ($arg:expr) => (
        match $arg {
            true  => fail!("assertion failed: {:s} sould be false", stringify!($arg)),
            false => {}
        }
    );
)