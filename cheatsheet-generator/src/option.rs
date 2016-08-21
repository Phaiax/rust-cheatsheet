use ::{References,Group,sel,MethodLine,Doc};

pub fn make(r : &mut References, mut section : &mut Group) {

   // Refernce: add_method_line(id, methodname, format, details)

    r.option.add_doc_by_element("option.option", sel("section#main div.docblock"));

    MethodLine::new().text( "let foo : Option<T> = Some(T::new());")
                     .finish(&mut r.option, &mut section);

    MethodLine::new().text( "                           = None;")
                     .finish(&mut r.option, &mut section);

    // If
    section.add_section("If");

    MethodLine::new().a_add_docs("is_some")
                     .text( "  .is_some();")
                     .finish(&mut r.option, &mut section);

    MethodLine::new().a_add_docs("is_none")
                     .text( "  .is_none();")
                     .details("")
                     .finish(&mut r.option, &mut section);
    // &
    section.add_section("&amp;");

    MethodLine::new().a_add_docs("as_ref")
                     .text( "  .as_ref();")
                     .details("-&gt; Option&lt;&amp;T&gt;")
                     .finish(&mut r.option, &mut section);

    MethodLine::new().a_add_docs("as_mut")
                     .text( "  .as_mut();")
                     .details("-&gt; Option&lt;&amp;mut T&gt;")
                     .finish(&mut r.option, &mut section);

    MethodLine::new().a_add_docs("cloned")
                     .text( "  .cloned();")
                     .details("Option&lt;&T&gt; -&gt; Option&lt;T&gt; if T:Clone")
                     .finish(&mut r.option, &mut section);

    MethodLine::new().a_add_docs("iter") .text( "  .iter")
                     .a_add_docs("iter_mut") .span( "_mut") .text("();")
                     .details("Iter with 1 or 0 elements")
                     .finish(&mut r.option, &mut section);

    // T
    section.add_section("Retrieve T");

    MethodLine::new().a_add_docs("unwrap")
                     .text( "  .unwrap();")
                     .details("-&gt; T or panic")
                     .finish(&mut r.option, &mut section);

    MethodLine::new().a_add_docs("expect")
                     .text( "  .expect(msg);")
                     .details("-&gt; T or panic(msg)")
                     .finish(&mut r.option, &mut section);

    MethodLine::new().a_add_docs("unwrap_or")
                     .text( "  .unwrap_or(default:T);")
                     .details("-&gt; T")
                     .finish(&mut r.option, &mut section);

    MethodLine::new().a_add_docs("unwrap_or_default")
                     .text( "  .unwrap_or_default();")
                     .details("-&gt; T, if T:Default")
                     .finish(&mut r.option, &mut section);

    MethodLine::new().a_add_docs("unwrap_or_else")
                     .text( "  .unwrap_or_else(|| -&gt; T);")
                     .details("-&gt; T")
                     .finish(&mut r.option, &mut section);

    MethodLine::new().a_add_docs("take")
                     .text( "mutableopt.take();")
                     .details("-&gt; Option&lt;T&gt;, moves T out of mutableopt")
                     .finish(&mut r.option, &mut section);

    // map
    section.add_section("Manipulate (map)");

    MethodLine::new().a_add_docs("map")
                     .text( "  .map(|t| -&gt; U);")
                     .details("-> Option&lt;U&gt;")
                     .finish(&mut r.option, &mut section);

    MethodLine::new().a_add_docs("map_or")
                     .text( "  .map_or(default:U, |t| -&gt; U);")
                     .details("-> Option&lt;U&gt;")
                     .finish(&mut r.option, &mut section);

    MethodLine::new().a_add_docs("map_or_else")
                     .text( "  .map_or_else(|| default -&gt; U, |t| -&gt; U);")
                     .details("-> Option&lt;U&gt;")
                     .finish(&mut r.option, &mut section);

    // Result
    section.add_section("to Result<>");

    MethodLine::new().a_add_docs("ok_or")
                     .text( "  .ok_or(err:E);")
                     .details("-> Result&lt;T,E&gt;")
                     .finish(&mut r.option, &mut section);

    MethodLine::new().a_add_docs("ok_or_else")
                     .text( "  .ok_or_else(|| err -&gt; E);")
                     .details("-> Result&lt;T,E&gt;")
                     .finish(&mut r.option, &mut section);

    // and, or
    section.add_section("Boolean Combinations");

    MethodLine::new().a_add_docs("and")
                     .text( " a.and(b : Option&ltU&gt;);")
                     .details("b if a && b")
                     .finish(&mut r.option, &mut section);

    MethodLine::new().a_add_docs("and_then")
                     .text( " a.and_then(|| b -&gt; Option&lt;U&gt;);")
                     .details("b if a && b")
                     .finish(&mut r.option, &mut section);

    MethodLine::new().a_add_docs("or")
                     .text( " a.or(b : Option&ltT&gt;);")
                     .details("a if a else b")
                     .finish(&mut r.option, &mut section);

    MethodLine::new().a_add_docs("or_else")
                     .text( " a.or_else(|| b -&gt; Option&lt;T&gt;);")
                     .details("a if a else b")
                     .finish(&mut r.option, &mut section);



// fn is_some(&self) -> bool
// fn is_none(&self) -> bool
// fn as_ref(&self) -> Option<&T>
// fn as_mut(&mut self) -> Option<&mut T>
// fn expect(self, msg: &str) -> T
// fn unwrap(self) -> T
// fn unwrap_or(self, def: T) -> T
// fn unwrap_or_else<F>(self, f: F) -> T where F: FnOnce() -> T
// fn map<U, F>(self, f: F) -> Option<U> where F: FnOnce(T) -> U
// fn map_or<U, F>(self, default: U, f: F) -> U where F: FnOnce(T) -> U
// fn map_or_else<U, D, F>(self, default: D, f: F) -> U where D: FnOnce() -> U, F: FnOnce(T) -> U
// fn ok_or<E>(self, err: E) -> Result<T, E>
// fn ok_or_else<E, F>(self, err: F) -> Result<T, E> where F: FnOnce() -> E
// fn iter(&self) -> Iter<T>
// fn and<U>(self, optb: Option<U>) -> Option<U>
// fn and_then<U, F>(self, f: F) -> Option<U> where F: FnOnce(T) -> Option<U>
// fn or(self, optb: Option<T>) -> Option<T>
// fn or_else<F>(self, f: F) -> Option<T> where F: FnOnce() -> Option<T>
// fn take(&mut self) -> Option<T>
// impl<'a, T> Option<&'a T> where T: Clone
// fn cloned(self) -> Option<T>
// impl<T> Option<T> where T: Default
// fn unwrap_or_default(self) -> T




    // TRAITS
    section.add_section("Traits");

    MethodLine::new()
        .a("trait.Hash")                        .text("Hash") .br()
        .a_select_add_docs(Doc::Impl("hash".into()))   .span("hash() |") .br()
        .a("trait.Debug")                       .text("Debug") .br()
        .a_select_add_docs(Doc::Impl("fmt".into()))   .span("fmt() |") .br()
        .a("trait.Ord")                         .text("Ord") .br()
        .a_select_add_docs(Doc::Impl("cmp".into()))   .span("cmp() |") .br()
        .a("trait.Eq")                          .text("Eq |") .br()
        .a("trait.PartialOrd")                  .text("PartialOrd") .br()
        .a_select_add_docs(Doc::Impl("partial_cmp".into())).span("partial_cmp() lt() le() gt() ge() |") .br()
        .a("trait.PartialEq")                   .text("PartialEq") .br()
        .a_select_add_docs(Doc::Impl("eq".into())).span("eq() ne() |") .br()
        .a("trait.Copy")                        .text("Copy |") .br()
        .a("trait.Clone")                       .text("Clone") .br()
        .a_select_add_docs(Doc::Impl("clone".into())).span("clone() clone_from() |") .br()
        .a("trait.Default")                     .text("Default") .br()
        .a_select_add_docs(Doc::Impl("default".into()))   .span("default() |") .br()
        .a("trait.IntoIterator")                .text("IntoIterator") .br()
        .a_select_add_docs(Doc::Nav("into_iter".into(), "pr".into(), "pnnnnn".into())).span("into_iter() |") .br()
        .a("trait.FromIterator")                .text("FromIterator") .br()
        .a_select_add_docs(Doc::LastImpl("from_iter".into())).span("from_iter() |") .br()
        .finish(&mut r.option, &mut section);



}