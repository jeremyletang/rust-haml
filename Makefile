# The MIT License (MIT)
#
# Copyright (c) 2014 Jeremy Letang (letang.jeremy@gmail.com)
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

HAML_RS = src/bin/main.rs
LIB_HAML_RS = src/lib.rs
LIB_HAML_TEST_RS = src/test.rs
HAML_OUT_DIR = bin
LIBS_OUT_DIR = lib
LIB_HAML_TEST_OUT_DIR = test
DOC_OUT_DIR = doc
LIB_COLORIZE_RS = deps/colorize/lib.rs

all: lib_deps haml libhaml #test docs

haml: lib_deps libhaml
	mkdir -p $(HAML_OUT_DIR)
	rustc -L $(LIBS_OUT_DIR) --out-dir=$(HAML_OUT_DIR) $(HAML_RS)

test: libhaml
	mkdir -p $(LIB_HAML_TEST_OUT_DIR)
	rustc -L $(LIBS_OUT_DIR) --test -o $(LIB_HAML_TEST_OUT_DIR)/libhaml_tests $(LIB_HAML_RS)

libhaml:
	mkdir -p $(LIBS_OUT_DIR)
	rustc --out-dir=$(LIBS_OUT_DIR) $(LIB_HAML_RS)

lib_deps:
	mkdir -p $(LIBS_OUT_DIR)
	rustc --out-dir=$(LIBS_OUT_DIR) $(LIB_COLORIZE_RS)

docs:
	mkdir -p doc
	rustdoc -o $(DOC_OUT_DIR) $(LIB_HAML_RS)



clean:
	rm -rf $(HAML_OUT_DIR)
	rm -rf $(DOC_OUT_DIR)
	rm -rf $(LIBS_OUT_DIR)
	rm -rf $(LIB_HAML_TEST_OUT_DIR)
