
extern crate scraper;
extern crate hyper;
extern crate ego_tree;
extern crate sha2;
extern crate rand;

use std::path::Path;
use std::fs::File;
use std::io::{Read, Write};
use scraper::{Html, Selector};
use scraper::element_ref::ElementRef;
use hyper::Client;
use hyper::header::Connection;
use sha2::{Sha256, Digest};
use rand::random;

mod vector;
mod hashmap;
mod option;

fn main() {

    let mut r = References::new();
    let mut builder = Builder::new();

    let mut vector_box = Group::new("{VEC}");
    vector::make(&mut r, &mut vector_box);
    builder.append_group(vector_box);

    let mut hashmap_box = Group::new("{HASHMAP}");
    hashmap::make(&mut r, &mut hashmap_box);
    builder.append_group(hashmap_box);

    let mut option_box = Group::new("{OPTION}");
    option::make(&mut r, &mut option_box);
    builder.append_group(option_box);

    r.from_iter.add_doc_by_element("trait.FromIterator", sel("section#main"));
    r.into_iter.add_doc_by_element("trait.IntoIterator", sel("section#main"));
    r.partial_ord.add_doc_by_element("trait.PartialOrd", sel("section#main"));
    r.partial_eq.add_doc_by_element("trait.PartialEq", sel("section#main"));
    r.ord.add_doc_by_element("trait.Ord", sel("section#main"));
    r.eq.add_doc_by_element("trait.Eq", sel("section#main"));
    r.write.add_doc_by_element("trait.Write", sel("section#main"));
    r.hash.add_doc_by_element("trait.Hash", sel("section#main"));
    r.debug.add_doc_by_element("trait.Debug", sel("section#main"));
    r.copy.add_doc_by_element("trait.Copy", sel("section#main"));
    r.clone.add_doc_by_element("trait.Clone", sel("section#main"));
    r.default.add_doc_by_element("trait.Default", sel("section#main"));


    builder.append_docs(r);
    builder.write();

}

pub fn a0(id : &str) -> String {
    format!("<a data-doc=\"{}\">", &id)
}

pub fn a1(id : &str) -> String {
    format!("</a><a data-doc=\"{}\">", &id)
}

pub struct Group {
    replacement_key : String,
    pub buf : Vec<String>,
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

pub struct MethodLine {
    /// (docid, methodname)
    docids : Vec<(String, String)>,
    buf : String,
    code_closed : bool,
    link_open : bool,
    rarelyused : bool,
    tags : String,
}


impl MethodLine {
    /// Adds a new line to a
    /// ```html
    ///    <div>
    ///       <code><a data-doc=%id>  %line </a></code>
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
    pub fn new() -> MethodLine {
        let mut m = Self::no_code_tag();
        m.code_closed = false;
        m.buf.push_str("<code>");
        m
    }

    pub fn no_code_tag() -> MethodLine {
        MethodLine {
            docids : vec![],
            buf : String::with_capacity(300),
            code_closed : true,
            link_open : false,
            rarelyused : false,
            tags : "".into(),
        }
    }

    pub fn rarelyused(mut self) -> MethodLine {
        self.rarelyused = true;
        self
    }

    /// Adds text
    pub fn text(mut self, text : &str) -> MethodLine {
        self.buf.push_str(text);
        self
    }

    /// Adds text in <span>
    pub fn span(mut self, text : &str) -> MethodLine {
        self.buf.push_str("<span>");
        self.buf.push_str(text);
        self.buf.push_str("</span>");
        self
    }

    /// Adds a formated method "   .{name}();" and includes documentation
    pub fn single_method_with_doc(self, methodname : &str)  -> MethodLine {
        self.a_add_docs(methodname)
            .text(format!("  .{}();", methodname).as_ref())
    }

    /// Adds a link to some extern docs (e.g. Trait docs)
    pub fn a(mut self, docid : &str) -> MethodLine {
        self.close_link();
        self.buf.push_str("<a data-doc=\"");
        self.buf.push_str(docid);
        self.buf.push_str("\">");
        self.link_open = true;
        self
    }

    /// Start new link and include docs, specify id for later use
    pub fn a_add_docs_use_id(mut self, methodname : &str, docid : &str) -> MethodLine {
        self.docids.push((docid.into(), methodname.into()));
        self.a(docid)
    }

    /// Start new link and include docs
    pub fn a_add_docs(self, methodname : &str) -> MethodLine {
        let docid = Self::docid(methodname);
        self.a_add_docs_use_id(methodname, &docid)
    }

    fn docid(methodid : &str) -> String{
        format!("{}.{}", random::<u64>(), methodid)
    }

    /// Closes <code> tag if still open
    fn close_code(&mut self) {
        self.close_link();

        if ! self.code_closed {
            self.buf.push_str("</code>");
            self.code_closed = true;
        }
    }

    fn br(mut self) -> MethodLine {
        self.close_code();
        self.buf.push_str(" <code>");
        self.code_closed = false;
        self
    }

    /// Closes <a> tag if still open
    fn close_link(&mut self) {
        if self.link_open {
            self.buf.push_str("</a>");
            self.link_open = false;
        }
    }

    fn make_start_tag(&self) -> String {
        format!("<div class=\"{}\" data-tags=\"{}\">",
                 if self.rarelyused { "rarelyused" } else { "" },
                 &self.tags,
        )
    }

    /// Adds <details> part.
    pub fn details(mut self, details : &str)  -> MethodLine {
        self.close_code();
        self.buf.push_str("<details>");
        self.buf.push_str(details);
        self.buf.push_str("</details>");
        return self;
    }

    pub fn tags(mut self, tags : &str) -> MethodLine {
        self.tags = tags.into();
        self
    }

    /// Include docs
    pub fn doc(self, doc : &mut Reference) -> MethodLine {
        for ref docid_methodname in &self.docids {
            doc.add_doc_for_method(&docid_methodname.0, &docid_methodname.1);
        }
        self
    }

    /// Adds the resulting html to the group
    pub fn to(mut self, group : &mut Group) {
        self.close_code();
        group.buf.push(self.make_start_tag());
        self.buf.push_str("</div>");
        group.buf.push(self.buf);
    }

    /// like .doc(d).to(r)
    pub fn finish(self, mut doc : &mut Reference, mut group : &mut Group) {
        self.doc(&mut doc).to(&mut group);
    }
}


/// Opens `../index.template.html` and replaces
pub struct Builder {
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

    /// Adds a new collection of popups (aka detauled docs, type `Refernce`)
    /// that will be put into the template
    fn append_docs(&mut self, docs : References) {
        docs.append_to_builder(self);
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
pub struct Reference {
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

        // We want to convert this: https://doc.rust-lang.org/std/default/trait.Default.html
        // into                     https://doc.rust-lang.org/std/default/
        let l = url.len() - url.rsplit("/").next().unwrap().len();
        let url = &url[0..l];

        // Read the Response.
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        body.replace("href='", &format!("target='_blank' href='{}", url))
    }
}

pub struct Method {
    name : String,
    id : String
}

impl Method {
    pub fn doc(&self, doc : &mut Reference) {
        doc.add_doc_for_method(&self.id, &self.name)
    }
}


/// Shortcut for Selector::parse
pub fn sel(selector : &str) -> Selector {
    Selector::parse(selector).expect(&format!("Selector {} not found.", selector))
}



pub struct References {
    pub vector : Reference,
    pub vector_macro : Reference,
    pub into_iter : Reference,
    pub from_iter : Reference,
    pub partial_eq : Reference,
    pub partial_ord : Reference,
    pub eq : Reference,
    pub ord : Reference,
    pub write : Reference,
    pub iter : Reference,
    pub hm : Reference,
    pub option : Reference,
    pub hash : Reference,
    pub debug : Reference,
    pub copy : Reference,
    pub clone : Reference,
    pub default : Reference,
}


impl References {
    pub fn new() -> References {
        References {
            vector : Reference::new("https://doc.rust-lang.org/std/vec/struct.Vec.html"),
            vector_macro : Reference::new("https://doc.rust-lang.org/std/macro.vec!.html"),
            into_iter : Reference::new("https://doc.rust-lang.org/std/iter/trait.IntoIterator.html"),
            from_iter : Reference::new("https://doc.rust-lang.org/std/iter/trait.FromIterator.html"),
            partial_eq : Reference::new("https://doc.rust-lang.org/std/cmp/trait.PartialEq.html"),
            partial_ord : Reference::new("https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html"),
            eq : Reference::new("https://doc.rust-lang.org/std/cmp/trait.Eq.html"),
            ord : Reference::new("https://doc.rust-lang.org/std/cmp/trait.Ord.html"),
            write : Reference::new("https://doc.rust-lang.org/std/io/trait.Write.html"),
            iter : Reference::new("https://doc.rust-lang.org/std/slice/struct.Iter.html"),
            hm : Reference::new("https://doc.rust-lang.org/std/collections/struct.HashMap.html"),
            option : Reference::new("https://doc.rust-lang.org/std/option/enum.Option.html"),
            hash : Reference::new("https://doc.rust-lang.org/std/hash/trait.Hash.html"),
            debug : Reference::new("https://doc.rust-lang.org/std/fmt/trait.Debug.html"),
            copy : Reference::new("https://doc.rust-lang.org/std/marker/trait.Copy.html"),
            clone : Reference::new("https://doc.rust-lang.org/std/clone/trait.Clone.html"),
            default : Reference::new("https://doc.rust-lang.org/std/default/trait.Default.html"),

        }
    }

    pub fn append_to_builder(self, builder : &mut Builder) {
        builder.append_doc(self.vector);
        builder.append_doc(self.vector_macro);
        builder.append_doc(self.into_iter);
        builder.append_doc(self.from_iter);
        builder.append_doc(self.partial_eq);
        builder.append_doc(self.partial_ord);
        builder.append_doc(self.eq);
        builder.append_doc(self.ord);
        builder.append_doc(self.write);
        builder.append_doc(self.iter);
        builder.append_doc(self.hm);
        builder.append_doc(self.option);
        builder.append_doc(self.hash);
        builder.append_doc(self.debug);
        builder.append_doc(self.copy);
        builder.append_doc(self.clone);
        builder.append_doc(self.default);
    }
}


