
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


    // Refernce: add_method_line(id, methodname, format, details)

    vector_box.add_method_line("vec.new", "new", Some("let mut vec: Vec&lt;T&gt; = Vec::new();"), "")
              .doc(&mut vector_ref);
    vector_box.add_method_line("vec.with_capacity", "with_capacity",
                               Some("            = Vec::with_capacity();"),
                               "").doc(&mut vector_ref);
    vector_box.add_method_line("vec.initmacro", "vec!",
                               Some("            = vec![];"), "");
    //vector_macro_ref.add_doc_by_element_range("vec.initmacro", sel("section#main"), sel("section.search"));

    //builder.add_link_line("            = vec![];", "", "https://doc.rust-lang.org/std/macro.vec!.html");
    //builder.add_line_customdoc("vec[3];", "", sel("#indexing"), sel("#slicing"));
    //builder.add_method_line("len", "", Some("vec.len()"));
    //builder.add_method_line("first", "-&gt; Option", Some("   .first<span>_mut</span>() .last<span>_mut</span>()"));
    //builder.add_method_line("get", "-&gt; Option", Some("   .get<span>_mut</span>()"));
    //builder.add_method_line("is_empty", "", None);


    let mut builder = Builder::new();
    builder.append_doc(vector_ref);
    builder.append_doc(vector_macro_ref);
    builder.append_group(vector_box);
    builder.write();

    //println!("{:?}", doc);
    //.as_element().expect("element");
    //assert!(doc.has_class("docblock"));

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
    fn add_method_line(&mut self,
                       id : &str,
                       methodname : &str,
                       format : Option<&str>,
                       details : &str) -> Method {

        let line_template = match format {
            Some(ref s) => s.to_string(),
            None        => "   .{1}();".to_string()
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
    fn add_link_line(&mut self, descr : &str, details : &str,  href : &str) {
        let line = "<div><a target=\"_blank\" href=\"{3}\"><code>{1}</code></a><details>{2}</details></div>\n";
        let line = line.replace("{1}", descr);
        let line = line.replace("{2}", details);
        let line = line.replace("{3}", href);
        self.buf.push(line);
    }

    /// ```html
    ///    <div>
    ///       <code> %descr </code>
    ///       <details> %details </details>
    ///    </div>
    /// ```
    fn add_line_customdoc(&mut self, descr : &str, details : &str, ) {
        let line = "<div><code>{1}</code><details>{2}</details></div>\n";
        let line = line.replace("{1}", descr);
        let line = line.replace("{2}", details);
        self.buf.push(line);

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
            Err(e) => None,
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
        let methoddata = self.document.select(&s).nth(0).expect("Method __ not found.");

        // select <a class="fnname"> within that <h4> and double check function name
        let s = sel("a.fnname");
        let fnname = methoddata.select(&s).nth(0).unwrap().text().nth(0).unwrap();
        assert_eq!(method, fnname);
        println!("{:?}", methoddata.value());

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
    Selector::parse(selector).unwrap()
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