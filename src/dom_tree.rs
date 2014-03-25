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

extern crate collections;

use std::vec::Vec;
use collections::HashMap;
use std::fmt;

#[deriving(Clone, Show, Eq, Ord)]
pub struct ElementId(Vec<i32>);

#[deriving(Clone, Show, Eq, Ord)]
pub enum TagType {
    Block,
    Inline,
    Content,
    Header,
    Root
}

#[deriving(Clone, Eq)]
pub struct DomTree {
    priv root: DomElement,
    priv cur_elt_id: ElementId,
    priv next_id: ElementId
}

#[deriving(Clone, Eq, Show)]
pub struct DomElement {
    priv parent: ElementId,
    priv childs: Vec<DomElement>,
    priv attributes: HashMap<~str, Vec<~str>>,
    priv tag: ~str,
    priv content: ~str,
    priv tag_type: TagType
}

impl DomTree {
    pub fn new() -> DomTree {
        DomTree {
            root: DomElement::root(),
            cur_elt_id: ElementId(vec!(0)),
            next_id: ElementId(vec!(0))
        }
    }

    pub fn get_elt<'a>(&'a self, elt_id: ElementId) -> Option<&'a DomElement> {
        let ElementId(mut tree_path) = elt_id.clone();
        match tree_path.shift() {
            Some(_) => rec_get_elt(&self.root, tree_path),
            None => None
        }
    }

    pub fn set_current_elt(&mut self, id: ElementId) {
        self.cur_elt_id = id
    }

    pub fn get_current_elt(&self) -> ElementId {
        self.cur_elt_id.clone()
    }

    pub fn insert(&mut self, mut elt: DomElement) -> Option<ElementId> {
        let ElementId(mut n) = self.cur_elt_id.clone();
        elt.parent = self.cur_elt_id.clone();
         match n.shift() {
            Some(_) => {
                let tmp_pos = rec_insert_elt(&mut self.root, elt, n);
                let ElementId(mut r) = self.cur_elt_id.clone();
                r.push(tmp_pos);
                self.cur_elt_id = ElementId(r.clone());
                Some(ElementId(r))
            },
            None => None
        }
    }

    pub fn insert_inline(&mut self, elt: DomElement) -> Option<ElementId> {
        self.insert(elt);
        self.back();
        Some(self.cur_elt_id.clone())
    }

    pub fn back(&mut self) {
        let ElementId(mut tree_path) = self.cur_elt_id.clone();
        tree_path.pop();
        self.cur_elt_id = ElementId(tree_path);
    }
}

fn rec_get_elt<'a>(elt: &'a DomElement,
                   mut tree_path: Vec<i32>) -> Option<&'a DomElement> {
    match tree_path.shift() {
        Some(idx) => rec_get_elt(elt.get_childs().get(idx as uint), tree_path),
        None => Some(elt)
    }
}

fn rec_insert_elt(elt: &mut DomElement,
                  new_elt: DomElement,
                  mut tree_path: Vec<i32>) -> i32 {
    match tree_path.shift() {
        Some(idx) => rec_insert_elt(elt.get_mut_childs().get_mut(idx as uint),
                                    new_elt,
                                    tree_path),
        None => elt.add_child(new_elt)
    }
}

impl DomElement {
    pub fn root() -> DomElement {
        DomElement {
            parent: ElementId(vec!(0)),
            childs: Vec::new(),
            attributes: HashMap::new(),
            tag: ~"",
            content: ~"",
            tag_type: Root
        }
    }

    pub fn new(tag: ~str) -> DomElement {
        DomElement {
            parent: ElementId(vec!(0)),
            childs: Vec::new(),
            attributes: HashMap::new(),
            tag: tag,
            content: ~"",
            tag_type: Block
        }
    }

    pub fn new_header(header: ~str) -> DomElement {
        DomElement {
            parent: ElementId(vec!(0)),
            childs: Vec::new(),
            attributes: HashMap::new(),
            tag: ~"",
            content: header,
            tag_type: Header
        }
    }

    pub fn new_content(content: ~str) -> DomElement {
        DomElement {
            parent: ElementId(vec!(0)),
            childs: Vec::new(),
            attributes: HashMap::new(),
            tag: ~"",
            content: content,
            tag_type: Content
        }
    }

    pub fn new_inline(tag: ~str,
                      content: ~str) -> DomElement {
        DomElement {
            parent: ElementId(vec!(0)),
            childs: Vec::new(),
            attributes: HashMap::new(),
            tag: tag,
            content: content,
            tag_type: Inline
        }
    }

    pub fn had_child(&self) -> bool {
        self.childs.len() != 0
    }

    pub fn get_childs<'a>(&'a self) -> &'a Vec<DomElement> {
        &self.childs
    }

    pub fn get_mut_childs<'a>(&'a mut self) -> &'a mut Vec<DomElement> {
        &mut self.childs
    }

    pub fn get_parent_id(&self) -> ElementId {
        self.parent.clone()
    }

    pub fn add_child(&mut self, elt: DomElement) -> i32 {
        self.childs.push(elt);
        (self.childs.len() - 1) as i32
    }
}

fn rec_show(elt: &DomElement,
            f: &mut fmt::Formatter,
            indent: ~str) -> fmt::Result {
    let mut res: fmt::Result = Ok(());
    for e in elt.get_childs().iter() {
        try!(write!(f.buf, "{}", indent));
        match e.tag_type {
            Content => try!(write!(f.buf, "{}\n", e.content)),
            Inline  => try!(write!(f.buf, "[tag: {}] {}\n", e.tag, e.content)),
            Block   => try!(write!(f.buf, "[tag: {}]\n", e.tag)),
            _       => {}
        }
        res = rec_show(e, f, indent + "  ");
    }
    res
}

impl fmt::Show for DomTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        rec_show(&self.root, f, ~"")
    }
}
