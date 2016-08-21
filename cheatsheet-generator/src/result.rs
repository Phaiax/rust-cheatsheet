use ::{References,Group,sel,MethodLine};

pub fn make(r : &mut References, mut section : &mut Group) {

   // Refernce: add_method_line(id, methodname, format, details)

    r.result.add_doc_by_element("result.result", sel("section#main div.docblock"));

    MethodLine::new().text( "let foo : Result<T,E> = Ok(T::new());")
                     .finish(&mut r.result, &mut section);

    MethodLine::new().text( "                           = Err(E::new());")
                     .finish(&mut r.result, &mut section);

    // If
    section.add_section("If");

    MethodLine::new().a_add_docs("is_ok")
                     .text( "  .is_ok();")
                     .finish(&mut r.result, &mut section);

    MethodLine::new().a_add_docs("is_err")
                     .text( "  .is_err();")
                     .details("")
                     .finish(&mut r.result, &mut section);
    // &
    section.add_section("&amp;");

    MethodLine::new().a_add_docs("as_ref")
                     .text( "  .as_ref();")
                     .details("-&gt; Result&lt;&amp;T, &amp;E&gt;")
                     .finish(&mut r.result, &mut section);

    MethodLine::new().a_add_docs("as_mut")
                     .text( "  .as_mut();")
                     .details("-&gt; Option&lt;&amp;mut T, &amp;mut E&gt;")
                     .finish(&mut r.result, &mut section);

    MethodLine::new().a_add_docs("iter") .text( "  .iter")
                     .a_add_docs("iter_mut") .span( "_mut") .text("();")
                     .details("Iter with 1 or 0 elements")
                     .finish(&mut r.result, &mut section);

    // T
    section.add_section("Retrieve T");

    MethodLine::new().a_add_docs("unwrap")
                     .text( "  .unwrap();")
                     .details("-&gt; T or panic; if E:Debug")
                     .finish(&mut r.result, &mut section);

    MethodLine::new().a_add_docs("expect")
                     .text( "  .expect(msg);")
                     .details("-&gt; T or panic(msg); if E:Debug")
                     .finish(&mut r.result, &mut section);

    MethodLine::new().a_add_docs("unwrap_or")
                     .text( "  .unwrap_or(default:T);")
                     .details("-&gt; T")
                     .finish(&mut r.result, &mut section);

    MethodLine::new().a_add_docs("unwrap_or_else")
                     .text( "  .unwrap_or_else(|err| default -&gt; T);")
                     .details("-&gt; T")
                     .finish(&mut r.result, &mut section);

    // E
    section.add_section("Retrieve E");

    MethodLine::new().a_add_docs("unwrap_err")
                     .text( "  .unwrap_err();")
                     .details("-&gt; E; if T:Debug")
                     .finish(&mut r.result, &mut section);


    // map
    section.add_section("Manipulate (map)");

    MethodLine::new().a_add_docs("map")
                     .text( "  .map(|t| -&gt; U);")
                     .details("-> Result&lt;U,E&gt;")
                     .finish(&mut r.result, &mut section);

    MethodLine::new().a_add_docs("map_err")
                     .text( "  .map_err(|e| -&gt; F);")
                     .details("-> Result&lt;T,F&gt;")
                     .finish(&mut r.result, &mut section);

    // Option
    section.add_section("to Option<>");

    MethodLine::new().a_add_docs("ok")
                     .text( "  .ok();")
                     .details("-> Option&lt;T&gt;")
                     .finish(&mut r.result, &mut section);

    MethodLine::new().a_add_docs("err")
                     .text( "  .err();")
                     .details("-> Option&lt;E&gt;")
                     .finish(&mut r.result, &mut section);

    // and, or
    section.add_section("Boolean Combinations");

    MethodLine::new().a_add_docs("and")
                     .text( " a.and(b : Result&ltU,E&gt;);")
                     .details("b if a && b else first err")
                     .finish(&mut r.result, &mut section);

    MethodLine::new().a_add_docs("and_then")
                     .text( " a.and_then(|| b -&gt; Result&lt;U,E&gt;);")
                     .details("b if a && b else first error")
                     .finish(&mut r.result, &mut section);

    MethodLine::new().a_add_docs("or")
                     .text( " a.or(b : Result&ltT,E&gt;);")
                     .details("a if a else b")
                     .finish(&mut r.result, &mut section);

    MethodLine::new().a_add_docs("or_else")
                     .text( " a.or_else(|| b -&gt; Result&lt;T,E&gt;);")
                     .details("a if a else b")
                     .finish(&mut r.result, &mut section);




    // TRAITS
    section.add_section("Traits");

    MethodLine::new()
        .a("trait.Hash")        .text("Hash") .br()
        .a_add_docs("hash")     .span("+") .br()
        .a_add_docs("hash_slice")     .span("+,") .br()
        .a("trait.Debug")        .text("Debug") .br()
        .a_add_docs("fmt")     .span("fmt() |") .br()
        .a("trait.Ord")        .text("Ord") .br()
        .a_add_docs("cmp")     .span("cmp() |") .br()
        .a("trait.Eq")        .text("Eq |") .br()
        .a("trait.PartialOrd") .text("PartialOrd") .br()
        .a_add_docs("partial_cmp")     .span("partial_cmp()") .br()
        .a_add_docs("lt")     .span("lt()") .br()
        .a_add_docs("le")     .span("le()") .br()
        .a_add_docs("gt")     .span("gt()") .br()
        .a_add_docs("ge")     .span("ge() |") .br()
        .a("trait.PartialEq") .text("PartialEq") .br()
        .a_add_docs("eq")     .span("eq()") .br()
        .a_add_docs("ne")     .span("ne() |") .br()
        .a("trait.Copy")        .text("Copy |") .br()
        .a("trait.Clone")        .text("Clone") .br()
        .a_add_docs("clone")     .span("clone()") .br()
        .a_add_docs("clone_from")     .span("clone_from(src) |") .br()
        .a("trait.IntoIterator") .text("IntoIterator") .br()
        .a_add_docs("into_iter")     .span("into_iter() |") .br()
        .a("trait.FromIterator") .text("FromIterator") .br()
        .a_add_docs("from_iter")     .span("from_iter()") .br()
        .finish(&mut r.result, &mut section);


}