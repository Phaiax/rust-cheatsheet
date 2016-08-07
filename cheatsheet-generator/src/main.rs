
extern crate scraper;
extern crate hyper;
extern crate ego_tree;

use std::fs::File;
use std::io::{Read, Write};
use scraper::{Html, Selector};
use scraper::element_ref::ElementRef;
use scraper::node::Node;
use hyper::Client;
use hyper::header::Connection;
use ego_tree::NodeRef;

fn main() {

    let mut vector_ref = Reference::new("https://doc.rust-lang.org/std/vec/struct.Vec.html");

    let mut vector_box = Group::new("{VECMETHODS}");
    vector_box.add_method_line("vec.new", "new", "", Some("let mut vec: Vec&lt;T&gt; = Vec::new();"));
    vector_ref.add_doc_for_method("vec.new", "new");
    //builder.add_method_line("with_capacity", "", Some("            = Vec::with_capacity();"));
    //builder.add_link_line("            = vec![];", "", "https://doc.rust-lang.org/std/macro.vec!.html");
    //builder.add_line_customdoc("vec[3];", "", sel("#indexing"), sel("#slicing"));
    //builder.add_method_line("len", "", Some("vec.len()"));
    //builder.add_method_line("first", "-&gt; Option", Some("   .first<span>_mut</span>() .last<span>_mut</span>()"));
    //builder.add_method_line("get", "-&gt; Option", Some("   .get<span>_mut</span>()"));
    //builder.add_method_line("is_empty", "", None);


    let mut builder = Builder::new();
    builder.append_doc(vector_ref);
    builder.append_group(vector_box);
    builder.write();

    //println!("{:?}", doc);
    //.as_element().expect("element");
    //assert!(doc.has_class("docblock"));



    println!("Hello, world! {:?}", "firsth1");
    //

}

struct Group {
    replacement_key : String,
    buf : Vec<String>,
}

impl Group {
    fn new(replacement_key : &str) -> Group{
        Group {
            replacement_key : replacement_key.to_string(),
            buf : Vec::with_capacity(1000),
        }
    }

    fn add_method_line(&mut self,
                       id : &str,
                       methodname : &str,
                       details : &str,
                       format : Option<&str>) {

        let line_template = match format {
            Some(ref s) => s.to_string(),
            None        => "   .{1}();".to_string()
        };
        let line = line_template.replace("{1}", &methodname);
        let line = "<div><a data-doc=\"{0}\"><code>{1}</code></a><details>{2}</details></div>\n    ".replace("{1}", &line);
        let line = line.replace("{2}", &details);
        let line = line.replace("{0}", &id);
        self.buf.push(line);
    }

    fn add_link_line(&mut self, descr : &str, details : &str,  href : &str) {
        let line = "<div><a target=\"_blank\" href=\"{3}\"><code>{1}</code></a><details>{2}</details></div>\n";
        let line = line.replace("{1}", descr);
        let line = line.replace("{2}", details);
        let line = line.replace("{3}", href);
        self.buf.push(line);
    }

    fn add_line_customdoc(&mut self, descr : &str, details : &str, ) {
        let line = "<div><code>{1}</code><details>{2}</details></div>\n";
        let line = line.replace("{1}", descr);
        let line = line.replace("{2}", details);
        self.buf.push(line);

    }
}

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

    fn append_doc(&mut self, doc : Reference) {
        self.docs.push(doc);
    }

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

    fn str(buf : &Vec<String>) -> String {
        let mut size = 0;
        for s in buf {
            size += s.len();
        }
        let mut cat = String::with_capacity(size);
        for s in buf {
            cat.push_str(&s);
        }
        cat
    }

    fn docs2str(&mut self) -> String {
        let mut size = 0;
        {
            let dref : &[Reference] = self.docs.as_ref();
            for d in dref {
                let href : &[String] = d.html.as_ref();
                for h in href {
                    size += h.len();
                }
            }
        }
        let mut cat = String::with_capacity(size);
        for d in self.docs.drain(..) {
            for h in d.html {
                cat.push_str(&h);
            }
        }
        cat
    }

    fn get_template() -> String {
        let mut template_file = File::open("../index.template.html").expect("../index.template.html not found");
        let mut contents = String::with_capacity(template_file.metadata().unwrap().len() as usize);
        template_file.read_to_string(&mut contents).unwrap();
        contents
    }

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


#[derive(Debug)]
struct Method {
    name : String,
    declaration : String,
    docblock : String
}

struct Reference {
    document : Html,
    html : Vec<String>,
}

impl Reference {
    pub fn new(url : &str) -> Reference {
        let html = Self::fetch(url);
        Reference {
            document : Html::parse_document(&html),
            html : Vec::with_capacity(10000),
        }
    }

    /// returns all generated HTML
    pub fn get_html(&mut self) -> &Vec<String> {
        &self.html
    }

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
        self.push_outerdoc_div(id);
        self.get_html_recursive(&methoddata);
        self.get_html_recursive(&docblock);
        self.push_enddiv();
    }

    fn push_outerdoc_div(&mut self, id : &str) {
        self.html.push("<div class=\"outerdoc\" id=\"".to_string());
        self.html.push(id.to_string());
        self.html.push("\">".to_string());
    }

    fn push_enddiv(&mut self) {
        self.html.push("</div>".to_string());
    }

    fn push_docblock_div(&mut self) {
        self.html.push("<div class=\"docblock\">".to_string());
    }

    /// Gets the HTML code from element `start` to element `end`, excluding end
    /// Useful for retrieving a paragraph within a larger section
    pub fn add_doc_by_element_range(&mut self, id : &str,
                                    start : Selector, end : Selector) {
        let start : ElementRef = self.document.select(&start).next().unwrap();
        let end : ElementRef = self.document.select(&end).next().unwrap();

        self.push_outerdoc_div(id);
        self.push_docblock_div();

        let mut buf = Vec::<String>::with_capacity(1000);
        self.get_html_recursive(&start);
        for next in start.next_siblings() {
            if let Some(nextref) = ElementRef::wrap(next) {
                if &nextref == &end {
                    break;
                }
            }
            self.get_html_node(next);
        }

        self.push_enddiv();
        self.push_enddiv();
    }

    /// Get the HTML code recursive. Helper for `get_html`.
    /// Writes the code into a new String and appends that string at the end of `buf`.
    fn get_html_recursive(&mut self, elem : &ElementRef) {

        let startag = format!("{:?}", elem.value());
        let startag = startag.replace("href=\"../../", "target=\"_blank\" href=\"https://doc.rust-lang.org/");
        self.html.push(startag);

        for c in elem.children() {
            // c : NodeRef<Node>
            self.get_html_node(c);
        }

        self.html.push(format!("</{}>", elem.value().name()));
    }

    /// Get html code by the `NodeRef`, not the `ElementRef`.
    /// Writes the code into a new String and appends that string at the end of `buf`.
    /// A node can be text or another ElementRef.
    fn get_html_node(&mut self, c : NodeRef<Node>) {
        let n : &Node = c.value();
        if n.is_document() {
            panic!("Unimplemented");
        } else if n.is_fragment() {
            panic!("Unimplemented");
        } else if n.is_doctype() {
            panic!("Unimplemented");
        } else if n.is_comment() {

        } else if n.is_text() {
            let t = n.as_text().unwrap();
            self.html.push(String::from_utf8_lossy(&t.as_bytes()).into_owned().replace("\n", "\n    "));
        } else if n.is_element() {
            //let e = c.as_element().unwrap();
            let e = ElementRef::wrap(c).unwrap();
            self.get_html_recursive(&e);
        }
    }


    /// Fetch a URL and saves it into
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
