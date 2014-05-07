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
pub struct ItemId(Vec<i32>);

#[deriving(Clone, Show, Eq, Ord)]
pub enum TagType {
    Block,
    Inline,
    PlainText,
    Header,
    HamlComment,
    HtmlComment,
    Root
}

#[deriving(Clone, Eq)]
pub struct DomTree {
    root: Item,
    cur_elt_id: ItemId,
    next_id: ItemId
}

#[deriving(Clone, Eq, Show)]
pub struct Item {
    parent: ItemId,
    childs: Vec<Item>,
    attributes: HashMap<~str, Vec<~str>>,
    tag: ~str,
    content: ~str,
    tag_type: TagType
}

impl DomTree {
    pub fn new() -> DomTree {
        DomTree {
            root: Item::root(),
            cur_elt_id: ItemId(vec!(0)),
            next_id: ItemId(vec!(0))
        }
    }

    pub fn get_elt<'a>(&'a self, elt_id: ItemId) -> Option<&'a Item> {
        let ItemId(mut tree_path) = elt_id.clone();
        match tree_path.shift() {
            Some(_) => rec_get_elt(&self.root, tree_path),
            None => None
        }
    }

    pub fn set_current_elt(&mut self, id: ItemId) {
        self.cur_elt_id = id
    }

    pub fn get_current_elt(&self) -> ItemId {
        self.cur_elt_id.clone()
    }

    pub fn insert(&mut self, mut elt: Item) -> Option<ItemId> {
        let ItemId(mut n) = self.cur_elt_id.clone();
        elt.parent = self.cur_elt_id.clone();
         match n.shift() {
            Some(_) => {
                let tmp_pos = rec_insert_elt(&mut self.root, elt, n);
                let ItemId(mut r) = self.cur_elt_id.clone();
                r.push(tmp_pos);
                self.cur_elt_id = ItemId(r.clone());
                Some(ItemId(r))
            },
            None => None
        }
    }

    pub fn insert_and_back(&mut self, elt: Item) -> Option<ItemId> {
        self.insert(elt);
        self.back();
        Some(self.cur_elt_id.clone())
    }

    pub fn back(&mut self) {
        let ItemId(mut tree_path) = self.cur_elt_id.clone();
        tree_path.pop();
        self.cur_elt_id = ItemId(tree_path);
    }

    pub fn get_current_lvl(&self) -> u32 {
        let ItemId(tree_path) = self.cur_elt_id.clone();
        (tree_path.len() - 1) as u32
    }
}

fn rec_get_elt<'a>(elt: &'a Item,
                   mut tree_path: Vec<i32>) -> Option<&'a Item> {
    match tree_path.shift() {
        Some(idx) => rec_get_elt(elt.get_childs().get(idx as uint), tree_path),
        None => Some(elt)
    }
}

fn rec_insert_elt(elt: &mut Item,
                  new_elt: Item,
                  mut tree_path: Vec<i32>) -> i32 {
    match tree_path.shift() {
        Some(idx) => rec_insert_elt(elt.get_mut_childs().get_mut(idx as uint),
                                    new_elt,
                                    tree_path),
        None => elt.add_child(new_elt)
    }
}

impl Item {
    pub fn root() -> Item {
        Item {
            parent: ItemId(vec!(0)),
            childs: Vec::new(),
            attributes: HashMap::new(),
            tag: "".to_owned(),
            content: "".to_owned(),
            tag_type: Root
        }
    }

    pub fn haml_comment() -> Item {
        Item {
            parent: ItemId(vec!(0)),
            childs: Vec::new(),
            attributes: HashMap::new(),
            tag: "".to_owned(),
            content: "".to_owned(),
            tag_type: HamlComment
        }
    }

    pub fn html_comment(content: ~str) -> Item {
        Item {
            parent: ItemId(vec!(0)),
            childs: Vec::new(),
            attributes: HashMap::new(),
            tag: "".to_owned(),
            content: content,
            tag_type: HtmlComment
        }
    }

    pub fn block(tag: ~str,
                 attributes: HashMap<~str, Vec<~str>>) -> Item {
        Item {
            parent: ItemId(vec!(0)),
            childs: Vec::new(),
            attributes: attributes,
            tag: if tag == "".to_owned() { "div".to_owned() } else { tag },
            content: "".to_owned(),
            tag_type: Block
        }
    }

    pub fn header(header: ~str) -> Item {
        Item {
            parent: ItemId(vec!(0)),
            childs: Vec::new(),
            attributes: HashMap::new(),
            tag: "".to_owned(),
            content: header,
            tag_type: Header
        }
    }

    pub fn plain_text(text: ~str) -> Item {
        Item {
            parent: ItemId(vec!(0)),
            childs: Vec::new(),
            attributes: HashMap::new(),
            tag: "".to_owned(),
            content: text,
            tag_type: PlainText
        }
    }

    pub fn inline(tag: ~str,
                  attributes: HashMap<~str, Vec<~str>>,
                  content: ~str) -> Item {
        Item {
            parent: ItemId(vec!(0)),
            childs: Vec::new(),
            attributes: attributes,
            tag: if tag == "".to_owned() { "div".to_owned() } else { tag },
            content: content,
            tag_type: Inline
        }
    }

    pub fn had_child(&self) -> bool {
        self.childs.len() != 0
    }

    pub fn get_childs<'a>(&'a self) -> &'a Vec<Item> {
        &self.childs
    }

    pub fn get_mut_childs<'a>(&'a mut self) -> &'a mut Vec<Item> {
        &mut self.childs
    }

    pub fn get_parent_id(&self) -> ItemId {
        self.parent.clone()
    }

    pub fn add_child(&mut self, elt: Item) -> i32 {
        self.childs.push(elt);
        (self.childs.len() - 1) as i32
    }
}

fn format_attribut(attributes: &HashMap<~str, Vec<~str>>) -> ~str {
    let mut fmt = StrBuf::new();
    for (at, values) in attributes.iter() {
        fmt.push_str(format!(" {}=\'", at));
        for v in values.iter() {
            fmt.push_str(format!("{} ", v));
        }
        if values.len() > 0 { unsafe { fmt.pop_byte(); } }
        fmt.push_str("\'");
    }
    fmt.into_owned()
}

fn rec_show(elt: &Item,
            f: &mut fmt::Formatter,
            indent: ~str) -> fmt::Result {
    let mut res: fmt::Result = Ok(());
    for e in elt.get_childs().iter() {
        let f_at = format_attribut(&e.attributes);
        match e.tag_type {
            PlainText   => try!(write!(f.buf, "{}{}\n", indent, e.content)),
            Inline      => try!(write!(f.buf, "{}<{}{}>{}</{}>\n", indent,
                                     e.tag, f_at, e.content, e.tag)),
            Block       => {
                if e.get_childs().len() == 0 {
                    try!(write!(f.buf, "{}<{}{}>", indent, e.tag, f_at));
                } else {
                    try!(write!(f.buf, "{}<{}{}>\n", indent, e.tag, f_at));
                }
                res = rec_show(e, f, indent + "  ");
                if e.get_childs().len() == 0 { try!(write!(f.buf, "</{}>\n", e.tag)); }
                else { try!(write!(f.buf, "{}</{}>\n", indent, e.tag)); }
            },
            HtmlComment => {
                if e.get_childs().len() == 0 { try!(write!(f.buf, "{}<!-- ", indent)); }
                else { try!(write!(f.buf, "{}<!--\n", indent)); }
                if e.content != "".to_owned() { try!(write!(f.buf, "{}", e.content)); }
                else { res = rec_show(e, f, indent + "  "); }
                if e.get_childs().len() == 0 { try!(write!(f.buf, " -->\n")); }
                else { try!(write!(f.buf, "{}-->\n", indent)); }
            }
            _           => {}
        }
    }
    res
}

impl fmt::Show for DomTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        rec_show(&self.root, f, "".to_owned())
    }
}
