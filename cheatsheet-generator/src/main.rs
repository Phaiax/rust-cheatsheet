
extern crate scraper;
extern crate hyper;
extern crate ego_tree;
extern crate sha2;

use std::path::Path;
use std::fs::File;
use std::io::{Read, Write};
use scraper::{Html, Selector};
use scraper::element_ref::ElementRef;
use hyper::Client;
use hyper::header::Connection;
use sha2::{Sha256, Digest};

fn main() {

    let mut vector_box = Group::new("{VECMETHODS}");
    let mut vector_ref = Reference::new("https://doc.rust-lang.org/std/vec/struct.Vec.html");
    let mut vector_macro_ref = Reference::new("https://doc.rust-lang.org/std/macro.vec!.html");
    let mut into_iter_ref = Reference::new("https://doc.rust-lang.org/std/iter/trait.IntoIterator.html");
    let mut from_iter_ref = Reference::new("https://doc.rust-lang.org/std/iter/trait.FromIterator.html");
    let mut partial_eq_ref = Reference::new("https://doc.rust-lang.org/std/cmp/trait.PartialEq.html");
    let mut partial_ord_ref = Reference::new("https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html");
    let mut eq_ref = Reference::new("https://doc.rust-lang.org/std/cmp/trait.Eq.html");
    let mut ord_ref = Reference::new("https://doc.rust-lang.org/std/cmp/trait.Ord.html");
    let mut write_ref = Reference::new("https://doc.rust-lang.org/std/io/trait.Write.html");
    let mut iter_ref = Reference::new("https://doc.rust-lang.org/std/slice/struct.Iter.html");

    // Refernce: add_method_line(id, methodname, format, details)

    // Intentionally missing functions for Vec
    // unsafe fn get_unchecked(&self, index: usize) -> &T
    // unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T
    // fn as_ptr(&self) -> *const T
    // fn as_mut_ptr(&mut self) -> *mut T
    // unsafe fn set_len(&mut self, len: usize)
    // unsafe fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Vec<T>

    // header
    vector_ref.add_doc_by_element("vec.vec", sel("section#main div.docblock"));

    // new
    vector_box.add_method_line("vec.new", "new", Some("let mut vec: Vec&lt;T&gt; = Vec::new();"), "")
              .doc(&mut vector_ref);
    // with_capacity
    vector_box.add_method_line("vec.with_capacity", "with_capacity",
                               Some("            = Vec::with_capacity();"),
                               "").doc(&mut vector_ref);

    // vec![]
    vector_box.add_method_line("vec.initmacro", "",
                               Some("            = vec![];"), "");
    vector_macro_ref.add_doc_by_element("vec.initmacro", sel("section#main"));

    // boxed array -> Vec
    vector_box.add_method_line("boxedvec.into_vec", "into_vec",
                               Some("            = boxedarray.into_vec()"), "")
                                .doc(&mut vector_ref);
    // From
    vector_box.add_method_line("vec.from", "from-1",
                               Some(&format!("            = Vec::from(slice|{}str|{}VecDeque|{}CString)",
                                a1("vec.from-str"),
                                a1("vec.from-vecdeque"),
                                a1("vec.from-cstring"),
                                )), "").doc(&mut vector_ref);
    vector_ref.add_doc_for_method("vec.from-str", "from-2");
    vector_ref.add_doc_for_method("vec.from-vecdeque", "from-3");
    vector_ref.add_doc_for_method("vec.from-cstring", "from-4");


    // clone
    vector_box.add_method_line("vec.clone", "clone",
                               Some("            = othervec.clone();"), "if T:Clone")
                               .doc(&mut vector_ref);


    // ACCESSING
    vector_box.add_section("Accessing");

    // v[]
    vector_box.add_method_line("vec.elementaccess", "",
                               Some("vec[3];"), "vec[1..3], vec[..3], vec[3..], vec[..]; vec[2] = a;");
    vector_ref.add_doc_by_element_range("vec.elementaccess", sel("#indexing"), sel("#slicing"));


    // len
    vector_box.add_method_line("vec.len", "len", Some("vec.len();"), "").doc(&mut vector_ref);

    // is_empty
    vector_box.add_method_line("vec.is_empty", "is_empty", None, "").doc(&mut vector_ref);

    // first_mut, last_mut
    vector_box.add_method_line("vec.first", "first",
                               Some(&format!("  .first{}<span>_mut</span>(); {}.last{}<span>_mut</span>();",
                                    a1("vec.first_mut"),
                                    a1("vec.last"),
                                    a1("vec.last_mut")
                                    )), "-&gt; Option").doc(&mut vector_ref);
    vector_ref.add_doc_for_method("vec.first_mut", "first_mut");
    vector_ref.add_doc_for_method("vec.last", "last");
    vector_ref.add_doc_for_method("vec.last_mut", "last_mut");

    // get_mut
    vector_box.add_method_line("vec.get", "get",
                               Some(&format!("  .get{}<span>_mut</span>(index);",
                                    a1("vec.get_mut"),
                                    )), "-&gt; Option").doc(&mut vector_ref);
    vector_ref.add_doc_for_method("vec.get_mut", "get_mut");


    // contains
    vector_box.add_method_line("vec.contains", "contains",
                               Some("  .contains(needle);"), "-&gt; bool")
                               .doc(&mut vector_ref);

    // find
    vector_box.add_method_line("iter.find", "",
                               Some("  .iter().find(|&amp;T| -> bool);"), "-&gt; Option");
    iter_ref.add_doc_for_method("iter.find", "find");

    // binary_search
    vector_box.add_method_line("vec.binary_search", "binary_search",
                               Some("  .binary_search(x:&amp;T);"), "-&gt; Result&lt;usize, usize&gt;<br>Ok(i): pos, Err(i): pos for insertion")
                               .doc(&mut vector_ref);

    // binary_search_by
    vector_box.add_method_line("vec.binary_search_by", "binary_search_by",
                               Some("  .binary_search_by(|&amp;T|->Ordering);"), "")
                               .doc(&mut vector_ref);

    // binary_search_by_key
    vector_box.add_method_line("vec.binary_search_by_key", "binary_search_by_key",
                               Some("  .binary_search_by_key(Key, |&amp;T|->Key);"), "")
                               .doc(&mut vector_ref);

    // ends_with
    vector_box.add_method_line("vec.ends_with", "ends_with",
                               Some("  .ends_with(needle);"), "")
                               .doc(&mut vector_ref);

    // starts_with
    vector_box.add_method_line("vec.starts_with", "starts_with",
                               Some("  .starts_with(needle);"), "")
                               .doc(&mut vector_ref);

    // ADDING
    vector_box.add_section("Adding");

    // push
    vector_box.add_method_line("vec.push", "push",
                               Some("  .push(3);"), "to end")
                               .doc(&mut vector_ref);

    // insert
    vector_box.add_method_line("vec.insert", "insert",
                               Some("  .insert(index, element);"), "")
                               .doc(&mut vector_ref);

    // extend
    vector_box.add_method_line("vec.extend", "extend",
                               Some("  .extend(iterable);"), "")
                               .doc(&mut vector_ref);

    // extend_from_slice
    vector_box.add_method_line("vec.extend_from_slice", "extend_from_slice",
                               Some("  .extend_from_slice(&amp;[T]);"), "")
                               .doc(&mut vector_ref);

    // append
    vector_box.add_method_line("vec.append", "append",
                                Some("  .append(other : Vec);"), "drains other")
                                .doc(&mut vector_ref);

    // clone_from
    vector_box.add_method_line("vec.clone_from", "clone_from",
                               Some("  .clone_from(&amp;Vec&lt;T&gt;);"), "overrides self, if T:Clone")
                               .doc(&mut vector_ref);

    // clone_from_slice
    vector_box.add_method_line("vec.clone_from_slice", "clone_from_slice",
                               Some("  .clone_from_slice(&amp;[T]);"), "if T:Clone")
                               .doc(&mut vector_ref);

    // copy_from_slice
    vector_box.add_method_line("vec.copy_from_slice", "copy_from_slice",
                               Some("  .copy_from_slice(&amp;[T]);"), "if T:Copy (use memcpy)")
                               .doc(&mut vector_ref);

    // REMOVING
    vector_box.add_section("Removing");

    // pop
    vector_box.add_method_line("vec.pop", "pop",
                               Some("  .pop();"), "removes last -> Option")
                               .doc(&mut vector_ref);

    // remove
    vector_box.add_method_line("vec.remove", "remove",
                               Some("  .remove(index);"), "-&gt; el, shifts left")
                               .doc(&mut vector_ref);

    // swap_remove
    vector_box.add_method_line("vec.swap_remove", "swap_remove",
                               Some("  .swap_remove(index);"), "-&gt; el, fills with last")
                               .doc(&mut vector_ref);

    // truncate
    vector_box.add_method_line("vec.truncate", "truncate",
                               Some("  .truncate(i);"), "cut until .len() = i")
                               .doc(&mut vector_ref);

    // drain
    vector_box.add_method_line("vec.drain", "drain",
                               Some("  .drain(range);"), "-&gt; iter that drains")
                               .doc(&mut vector_ref);

    // clear
    vector_box.add_method_line("vec.clear", "clear",
                               Some("  .clear();"), "")
                               .doc(&mut vector_ref);

    // retain
    vector_box.add_method_line("vec.retain", "retain",
                               Some("  .retain(|i| -&gt; bool);"), "in place")
                               .doc(&mut vector_ref);

    // dedup
    vector_box.add_method_line("vec.dedup", "dedup",
                               Some("  .dedup();"), "removes duplicates (if T:PartialEq)")
                               .doc(&mut vector_ref);

    // MANIPULATING
    vector_box.add_section("Manipulating");

    // sort
    vector_box.add_method_line("vec.sort", "sort",
                               Some("  .sort();"), "in place")
                               .doc(&mut vector_ref);

    // sort_by
    vector_box.add_method_line("vec.sort_by", "sort_by",
                               Some("  .sort_by(|&amp;T|->Ordering);"), "in place")
                               .doc(&mut vector_ref);

    // sort_by_key
    vector_box.add_method_line("vec.sort_by_key", "sort_by_key",
                               Some("  .sort_by_key(|&amp;T|->Key);"), "Key:Ordering")
                               .doc(&mut vector_ref);

    // reverse
    vector_box.add_method_line("vec.reverse", "reverse",
                               Some("  .reverse();"), "in place")
                               .doc(&mut vector_ref);

    // swap
    vector_box.add_method_line("vec.swap", "swap",
                               Some("  .swap(index1, index2);"), "")
                               .doc(&mut vector_ref);


    // TRANSFORMING
    vector_box.add_section("Transforming (Iter, as_, to_)");

    // iter
    vector_box.add_method_line("vec.iter", "iter",
                               Some(&format!("  .iter{}<span>_mut</span>();",
                                a1("vec.iter_mut"),
                                )), "<div title=\"borrows value\">-&gt;&<span>mut </span>T, keeps vector</div>")
                               .doc(&mut vector_ref);
    vector_ref.add_doc_for_method("vec.iter_mut", "iter_mut");

    // into_iter
    vector_box.add_method_line("vec.into_iter", "into_iter",
                               Some("  .into_iter();"), "<div title=\"transfers ownership\">-&gt;T, consumes vector</div>")
                               .doc(&mut vector_ref);

    // chunks
    vector_box.add_method_line("vec.chunks", "chunks",
                               Some(&format!("  .chunks{}<span>_mut</span>(cnk_sz);",
                                a1("vec.chunks_mut"),
                                )), "-&gt; iter over a non overlapping slice at a time")
                               .doc(&mut vector_ref);
    vector_ref.add_doc_for_method("vec.chunks_mut", "chunks_mut");

    // windows
    vector_box.add_method_line("vec.windows", "windows",
                               Some("  .windows(wnd_sz);"), "-&gt; iter over an overlapping slice at a time")
                               .doc(&mut vector_ref);
    vector_box.add_hr();

    // into_boxed_slice
    vector_box.add_method_line("vec.into_boxed_slice", "into_boxed_slice",
                               Some("  .into_boxed_slice();"), "-&gt; Box&lt;T&gt;")
                               .doc(&mut vector_ref);

    // as_ref
    vector_box.add_method_line("vec.as_ref", "as_ref",
                               Some("  .as_ref();"), "-&gt; &amp;[T] or &amp;Vec&lt;T&gt;")
                               .doc(&mut vector_ref);

    // to_vec
    vector_box.add_method_line("vec.to_vec", "to_vec",
                               Some("  .to_vec();"), "like clone(), if T:Clone")
                               .doc(&mut vector_ref);

    // as_slice
    vector_box.add_method_line("vec.as_slice", "as_slice",
                               Some(&format!("  .as{}<span>_mut</span>{}_slice();",
                                a1("vec.as_mut_slice"),
                                a1("vec.as_slice"))), "-&gt; &amp;<span>mut</span>[T]")
                               .doc(&mut vector_ref);
    vector_ref.add_doc_for_method("vec.as_mut_slice", "as_mut_slice");


    // MEMORY
    vector_box.add_section("Memory");

    // capacity
    vector_box.add_method_line("vec.capacity", "capacity",
                               Some("  .capacity();"), "")
                               .doc(&mut vector_ref);

    // reserve
    vector_box.add_method_line("vec.reserve", "reserve",
                               Some("  .reserve(100);"), "in addition to .len() or more")
                               .doc(&mut vector_ref);

    // reserve_exact
    vector_box.add_method_line("vec.reserve_exact", "reserve_exact",
                               Some("  .reserve_exact(100);"), "in addition to .len()")
                               .doc(&mut vector_ref);

    // shrink_to_fit
    vector_box.add_method_line("vec.shrink_to_fit", "shrink_to_fit",
                               Some("  .shrink_to_fit();"), "")
                               .doc(&mut vector_ref);

    // MEMORY
    vector_box.add_section("Split");


    // split_at
    vector_box.add_method_line("vec.split_at", "split_at",
                               Some(&format!("  .split_at{}<span>_mut</span>(mid);",
                                a1("vec.split_at_mut"),
                                )), "-&gt; (p1, p2), [mid] in 2nd part")
                               .doc(&mut vector_ref);
    vector_ref.add_doc_for_method("vec.split_at_mut", "split_at_mut");


    vector_box.add_method_line("vec.split", "split",
                               Some(&format!("  .split{}<span>_mut</span>(|&amp;T| -&gt; bool);",
                                    a1("vec.split_mut"),
                                    )), "").doc(&mut vector_ref);
    vector_ref.add_doc_for_method("vec.split_mut", "split_mut");


    vector_box.add_method_line("vec.splitn", "splitn",
                               Some(&format!("  .splitn{}<span>_mut</span>(n, |&amp;T| -&gt; bool); {}.rsplitn{}<span>_mut</span>(_);",
                                    a1("vec.splitn_mut"),
                                    a1("vec.rsplitn"),
                                    a1("vec.rsplitn_mut"),
                                    )), "-&gt; iter over <span>mutable</span> subslices,<br> seperated by ||-&gt;true, <span>at most n times</span>").doc(&mut vector_ref);

    vector_ref.add_doc_for_method("vec.splitn_mut", "splitn_mut");
    vector_ref.add_doc_for_method("vec.rsplitn", "rsplitn");
    vector_ref.add_doc_for_method("vec.rsplitn_mut", "rsplitn_mut");


    vector_box.add_method_line("vec.split_off", "split_off",
                               Some("  .split_off(mid);"), "-&gt; Vec; [mid] in 2nd part")
                               .doc(&mut vector_ref);

    // COMPARISION
    vector_box.add_section("Comparision");

    // cmp
    vector_box.add_method_line("vec.cmp", "cmp",
                               Some(&format!("  .cmp() {}.eq() {}.ne();",
                                a1("vec.eq"),
                                a1("vec.ne"),)),
                               "T: PartialEq")
                               .doc(&mut vector_ref);
    vector_ref.add_doc_for_method("vec.eq", "eq");
    vector_ref.add_doc_for_method("vec.ne", "ne");

    // lt
    vector_box.add_method_line("vec.lt", "lt",
                               Some(&format!("  .lt() {}.le() {}.gt() {}.ge();",
                                a1("vec.le"),
                                a1("vec.gt"),
                                a1("vec.ge"))),
                                "if T:PartialOrd")
                               .doc(&mut vector_ref);
    vector_ref.add_doc_for_method("vec.le", "le");
    vector_ref.add_doc_for_method("vec.gt", "gt");
    vector_ref.add_doc_for_method("vec.ge", "ge");

    vector_box.add_hr();

    // hash
    vector_box.add_method_line("vec.hash", "hash",
                               Some("  .hash(state: Hasher)"), "if T:Hash")
                               .doc(&mut vector_ref);

    // TRAITS
    vector_box.add_section("Traits");

    vector_box.add_line_customdoc(&format!("
        {}<code>From&lt;BinaryHeap&gt;</code>,
        {}<code>Borrow{}<span>Mut</span></code>,
        {}<code>Clone {}<span>+</span></code>,
        {}<code>Hash {}<span>+</span></code>,
        {}<code>Index{}<span>Mut</span></code>,
        {}<code>Deref{}<span>Mut</span></code>,
        {}<code>FromIterator</code>,
        {}<code>IntoIterator</code>,
        {}<code>Extend</code>,
        {}<code>PartialEq</code>,
        {}<code>PartialOrd</code>,
        {}<code>Eq</code>,
        {}<code>Ord</code>,
        {}<code>Drop</code>,
        {}<code>Default</code>,
        {}<code>Debug (if T:Debug)</code>,
        {}<code>AsRef</code>,
        {}<code>AsMut</code>,
        {}<code>From</code>,
        {}<code>Write</code>
        </a>",
        a0("vec.BinaryHeap-from"),
        a1("vec.borrow"),
        a1("vec.borrow_mut"),
        a1("vec.clone"),
        a1("vec.clone_from"),
        a1("vec.hash"),
        a1("vec.hash_slice"),
        a1("vec.index"),
        a1("vec.index_mut"),
        a1("vec.deref"),
        a1("vec.deref_mut"),
        a1("vec.FromIterator"),
        a1("vec.IntoIterator"),
        a1("vec.extend"),
        a1("vec.PartialEq"),
        a1("vec.PartialOrd"),
        a1("vec.Eq"),
        a1("vec.Ord"),
        a1("vec.Drop"),
        a1("vec.Default"),
        a1("vec.Debug"),
        a1("vec.AsRef"),
        a1("vec.AsMut"),
        a1("vec.From"),
        a1("vec.Write"),
        ), "" );

    vector_ref.add_doc_for_method("vec.BinaryHeap-from", "from");
    vector_ref.add_doc_for_method("vec.borrow", "borrow");
    vector_ref.add_doc_for_method("vec.borrow_mut", "borrow_mut");
    vector_ref.add_doc_for_method("vec.clone", "clone");
    vector_ref.add_doc_for_method("vec.clone_from", "clone_from");
    vector_ref.add_doc_for_method("vec.hash", "hash");
    vector_ref.add_doc_for_method("vec.hash_slice", "hash_slice");
    vector_ref.add_doc_for_method("vec.index", "index");
    vector_ref.add_doc_for_method("vec.index", "index");
    vector_ref.add_doc_for_method("vec.index_mut", "index_mut");
    vector_ref.add_doc_for_method("vec.deref", "deref");
    vector_ref.add_doc_for_method("vec.deref_mut", "deref_mut");
    from_iter_ref.add_doc_by_element("vec.FromIterator", sel("section#main"));
    into_iter_ref.add_doc_by_element("vec.IntoIterator", sel("section#main"));
    vector_ref.add_doc_for_method("vec.extend", "extend");
    partial_ord_ref.add_doc_by_element("vec.PartialOrd", sel("section#main"));
    partial_eq_ref.add_doc_by_element("vec.PartialEq", sel("section#main"));
    ord_ref.add_doc_by_element("vec.Ord", sel("section#main"));
    eq_ref.add_doc_by_element("vec.Eq", sel("section#main"));
    vector_ref.add_doc_for_method("vec.Drop", "drop");
    vector_ref.add_doc_for_method("vec.Default", "default");
    vector_ref.add_doc_for_method("vec.Debug", "fmt");
    vector_ref.add_doc_for_method("vec.AsRef", "as_ref");
    vector_ref.add_doc_for_method("vec.AsMut", "as_mut");
    vector_ref.add_doc_for_method("vec.From", "from-1");
    write_ref.add_doc_by_element("vec.Write", sel("section#main"));


    let mut builder = Builder::new();
    builder.append_doc(vector_ref);
    builder.append_doc(vector_macro_ref);
    builder.append_doc(into_iter_ref);
    builder.append_doc(from_iter_ref);
    builder.append_doc(partial_eq_ref);
    builder.append_doc(partial_ord_ref);
    builder.append_doc(eq_ref);
    builder.append_doc(ord_ref);
    builder.append_doc(write_ref);
    builder.append_group(vector_box);
    builder.write();

}

fn a0(id : &str) -> String {
    format!("<a data-doc=\"{}\">", &id)
}

fn a1(id : &str) -> String {
    format!("</a><a data-doc=\"{}\">", &id)
}

struct Group {
    replacement_key : String,
    buf : Vec<String>,
}

/// Represents a group of consecutive lines in the cheatsheet.
impl Group {
    fn new(replacement_key : &str) -> Group{
        Group {
            replacement_key : replacement_key.to_string(),
            buf : Vec::with_capacity(1000),
        }
    }

    /// Adds a new line to this group
    /// ```html
    ///    <div>
    ///       <a data-doc=%id> <code> %line </code> </a>
    ///       <details> %details </details>
    ///    </div>
    /// ```
    /// Where %line =
    /// ```ign
    ///                 "    .%methodname();"                if `format` == None
    ///                 %format.replace({1}, %methodname)    if `format` = Some(%format)
    /// ```
    /// Returns an object that has the method .doc(&mut Reference), that can be used to
    /// take care of the popup.
    pub fn add_method_line(&mut self,
                       id : &str,
                       methodname : &str,
                       format : Option<&str>,
                       details : &str) -> Method {

        let line_template = match format {
            Some(ref s) => s.to_string(),
            None        => "  .{1}();".to_string()
        };
        let line = line_template.replace("{1}", &methodname);
        let line = "<div><a data-doc=\"{0}\"><code>{1}</code></a><details>{2}</details></div>\n    ".replace("{1}", &line);
        let line = line.replace("{2}", &details);
        let line = line.replace("{0}", &id);
        self.buf.push(line);

        Method {
            name : methodname.to_owned(),
            id : id.to_owned()
        }
    }

    /// Link to external page
    ///
    /// ```html
    ///    <div>
    ///       <a href=%href> <code> %descr </code> </a>
    ///       <details> %details </details>
    ///    </div>
    /// ```
    pub fn add_link_line(&mut self, descr : &str, details : &str,  href : &str) {
        let line = "<div><a target=\"_blank\" href=\"{3}\"><code>{1}</code></a><details>{2}</details></div>\n";
        let line = line.replace("{1}", descr);
        let line = line.replace("{2}", details);
        let line = line.replace("{3}", href);
        self.buf.push(line);
    }

    /// ```html
    ///    <div>
    ///       %descr
    ///       <details> %details </details>
    ///    </div>
    /// ```
    pub fn add_line_customdoc(&mut self, descr : &str, details : &str, ) {
        let line = "<div>{1}<details>{2}</details></div>\n";
        let line = line.replace("{1}", descr);
        let line = line.replace("{2}", details);
        self.buf.push(line);
    }

    pub fn add_hr(&mut self) {
        self.buf.push("<hr><hr class=\"hide\">".to_string());
    }

    pub fn add_section(&mut self, title : &str) {
        self.add_hr();
        self.buf.push(format!("<h6>{}</h6>", title));
    }
}

/// Opens `../index.template.html` and replaces
struct Builder {
    template : String,
    docs : Vec<Reference>,
    groups : Vec<Group>,
}

impl Builder {
    fn new() -> Builder {
        Self::check_working_directory();
        Builder {
            template : Self::get_template(),
            docs : Vec::with_capacity(10),
            groups : Vec::with_capacity(10),
        }
    }

    /// Adds a new collection of popups (aka detauled docs, type `Refernce`)
    /// that will be put into the template
    fn append_doc(&mut self, doc : Reference) {
        self.docs.push(doc);
    }

    /// append a group
    fn append_group(&mut self, group : Group) {
        self.groups.push(group);
    }

    fn parse_template(&mut self) {
        for group in self.groups.drain(..) {
            self.template = self.template.replace(&group.replacement_key, &Self::str(&group.buf));
        }
        let docs_str = self.docs2str();
        self.template = self.template.replace("{{DOC}}", &docs_str);
    }

    /// Simple concat of all strings in a vector of strings
    fn str(buf : &Vec<String>) -> String {
        let size = buf.iter().fold(0, |size, part| size + part.len());
        let mut cat = String::with_capacity(size);
        for s in buf {
            cat.push_str(&s);
        }
        cat
    }

    /// Concats all html from all `Reference`es
    fn docs2str(&mut self) -> String {
        // calc total size
        let size = self.docs.iter().fold(0, |size, doc| size + doc.get_html_len());

        let mut cat = String::with_capacity(size);
        for d in self.docs.drain(..) {
            for h in d.get_html() {
                cat.push_str(&h);
            }
        }
        cat
    }

    /// Read file `../index.template.html`
    fn get_template() -> String {
        let mut template_file = File::open("../index.template.html").expect("../index.template.html not found");
        let mut contents = String::with_capacity(template_file.metadata().unwrap().len() as usize);
        template_file.read_to_string(&mut contents).unwrap();
        contents
    }

    /// Parses template and writes the processed version into `../index.html`
    pub fn write(&mut self) {
        self.parse_template();
        let mut html_file = File::create("../index.html").expect("../index.html not writable");
        html_file.write_all(self.template.as_bytes()).unwrap();
    }

    /// Check for this crates Cargo.toml. We want to write into the upper directory,
    /// and that directory should be the correct one.
    fn check_working_directory() {
        let mut cargo_toml = File::open("Cargo.toml").expect("Unexpected working directory. (Cargo.toml missing)");
        let mut contents = String::with_capacity(cargo_toml.metadata().unwrap().len() as usize);
        cargo_toml.read_to_string(&mut contents).unwrap();
        assert!(contents.contains("name = \"cheatsheet-generator\""));
    }



}


/// Fetches the current documentation from the online docs. Then prepares the
/// html for the popups. Each possible popup is created by calling
///     add_doc_for_method(id, method)
///     add_doc_by_element_range(id, start, end)
/// This class then generates the hidden <div>s that are later used for the
/// popups. The builder takes the html generated by each `Reference` instance
/// and puts it into the final template (`Builder::append_doc(reference)`)
/// Use `<a data-doc="id">` to access the popup. Some javascript code will
/// do the linking.
///
/// The generated html can be retrieved by using `get_html()`. It has the
/// following format:
///
/// ```html
///     <div class="outerdoc" id="<id>">
///         <h4> header from original doc </h4>
///         docdetails
///     </div>
/// ```
struct Reference {
    document : Html,
    html : Vec<String>,
}

impl Reference {

    /// Create doc popups from functions that can be found in `url`.
    /// e.g. url = https://doc.rust-lang.org/std/vec/struct.Vec.html
    pub fn new(url : &str) -> Reference {
        // create a Sha256 object
        let mut hasher = Sha256::new();
        hasher.input_str(url);
        let hash = hasher.result_str();

        let html = match Self::read_from_cache(&hash) {
            Some(h) => {
                println!("Cachehit: {} bytes from {}", h.len(), url);
                h
            },
            None => {
                let html = Self::fetch(url);
                Self::write_to_cache(&html, &hash);
                println!("Fetched {} bytes from {}", html.len(), url);
                html
            }
        };
        Reference {
            document : Html::parse_document(&html),
            html : Vec::with_capacity(10000),
        }
    }

    fn write_to_cache(html : &str, hash : &str) {
        let cachedir = Path::new("../cache");
        std::fs::create_dir(cachedir).ok();
        let mut cache_file = File::create(cachedir.join(hash)).expect("../cache/foobar not writable");
        cache_file.write_all(html.as_bytes()).unwrap();
    }

    fn read_from_cache(hash : &str) -> Option<String> {
        let cachedir = Path::new("../cache");
        match File::open(cachedir.join(hash)) {
            Ok(mut cache_file) => {
                let mut contents = String::with_capacity(cache_file.metadata().unwrap().len() as usize);
                cache_file.read_to_string(&mut contents).unwrap();
                Some(contents)
            },
            Err(_) => None,
        }

    }

    /// returns all generated HTML
    pub fn get_html(&self) -> &Vec<String> {
        &self.html
    }

    pub fn get_html_len(&self) -> usize {
        self.html.iter().fold(0, |size, part| size + part.len())
    }

    /// Searches within the Reference for the docs of method `method`.
    /// Then pushes the details to the outbuffer, enclosed in a div.outerdoc#id
    pub fn add_doc_for_method(&mut self, id : &str, method : &str) {
        // select <h4 id="method.methodname">
        let selector_str = format!("h4#method\\.{}", method);
        let s = sel(&selector_str);
        let methoddata = self.document.select(&s).nth(0).expect(
            &format!("Method {} not found.", method));

        // select <a class="fnname"> within that <h4> and double check function name
        let s = sel("a.fnname");
        let _fnname = methoddata.select(&s).nth(0).unwrap().text().nth(0).unwrap();
        //assert_eq!(method, fnname);

        // select first <div class="docblock"> after <h4 id="method.methodname">
        let selector_str = format!("h4#method\\.{} + div.docblock", method);
        let s = sel(&selector_str);
        let docblock : ElementRef = self.document.select(&s).nth(0).expect("No div.docblock found after h4#method\\.__");

        // surround with div.outerdoc#id
        self.html.push(Self::make_div_starttag("outerdoc", id));
        self.html.push(methoddata.html());
        self.html.push(docblock.html());
        self.html.push(Self::make_div_endtag());
        // self.get_html_recursive(&methoddata);
        // self.get_html_recursive(&docblock);
    }

    fn make_div_starttag(class : &str, id : &str) -> String {
        format!("<div class=\"{}\" id=\"{}\">", class, id)
    }

    fn make_div_endtag() -> String {
        "</div>".to_string()
    }

    /// Gets the HTML code from element `start` to element `end`, excluding end
    /// Useful for retrieving a paragraph within a larger section
    pub fn add_doc_by_element_range(&mut self, id : &str,
                                    start : Selector, end : Selector) {
        self.html.push(Self::make_div_starttag("outerdoc", id));
        self.html.push(Self::make_div_starttag("docblock", id));

        let start : ElementRef = self.document.select(&start).next().unwrap();
        let end : ElementRef = self.document.select(&end).next().unwrap();

        self.html.push(start.html());
        for next in start.next_siblings() {
            if let Some(nextref) = ElementRef::wrap(next) {
                if &nextref == &end {
                    break;
                }
                self.html.push(nextref.html());
            }
        }

        self.html.push(Self::make_div_endtag());
        self.html.push(Self::make_div_endtag());
    }

    /// Gets the inner html of `element`
    pub fn add_doc_by_element(&mut self, id : &str, element : Selector) {

        self.html.push(Self::make_div_starttag("outerdoc", id));
        self.html.push(Self::make_div_starttag("docblock", id));

        let element : ElementRef = self.document.select(&element).next().unwrap();
        self.html.push(element.inner_html());

        self.html.push(Self::make_div_endtag());
        self.html.push(Self::make_div_endtag());

    }

    /// Fetch a URL and return the Response as String
    fn fetch(url : &str) -> String {
        // Create a client.
        let client = Client::new();

        // Creating an outgoing request.
        let mut res = client.get(url)
            // set a header
            .header(Connection::close())
            // let 'er go!
            .send().unwrap();

        // Read the Response.
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        body
    }
}


/// Shortcut for Selector::parse
fn sel(selector : &str) -> Selector {
    Selector::parse(selector).expect(&format!("Selector {} not found.", selector))
}


struct Method {
    name : String,
    id : String
}

impl Method {
    fn doc(&self, doc : &mut Reference) {
        doc.add_doc_for_method(&self.id, &self.name)
    }
}