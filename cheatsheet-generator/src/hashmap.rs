use ::{References,Group,sel,MethodLine,Doc};

pub fn make(r : &mut References, mut section : &mut Group) {

   // Refernce: add_method_line(id, methodname, format, details)

    r.hm.add_doc_by_element("hm.hm", sel("section#main div.docblock"));
    section.add_line_customdoc("<code>use std::collections::HashMap;</code>", "");

    // new
    MethodLine::new().a_add_docs("new")
                     .text( "let mut foo: HashMap&lt;K, V&gt; = HashMap::new();")
                     .finish(&mut r.hm, &mut section);

    // with_capacity
    MethodLine::new().a_add_docs("with_capacity")
                     .text("            = HashMap::with_capacity();")
                     .details("K: Eq, Hash")
                     .finish(&mut r.hm, &mut section);

    MethodLine::new().a_add_docs_use_id("clone", "hm.clone")
                     .text("            = other.clone();")
                     .details("if V,K:Clone")
                     .finish(&mut r.hm, &mut section);

    // ACCESSING
    section.add_section("Access");

    // v[]
    MethodLine::new().a_add_docs_use_id("index", "hm.index")
                     .text("foo[key];")
                     .finish(&mut r.hm, &mut section);;
    //r.hm.add_doc_by_element_range("hm.elementaccess", sel("#indexing") sel("#slicing"));

    MethodLine::new().a_add_docs("len")
                     .text("foo.len();")
                     .finish(&mut r.hm, &mut section);

    MethodLine::new().a_add_docs("iter")        .text("  .iter")
                     .a_add_docs("iter_mut")    .text("<span>_mut</span>();")
                     .details("-&gt; iter over (&amp;K, &amp;<span>mut </span>V)")
                     .finish(&mut r.hm, &mut section);

    MethodLine::new().a_add_docs("into_iter")
                     .text("  .into_iter();")
                     .details("-&gt; Iter<span>Mut</span>")
                     .finish(&mut r.hm, &mut section);

    MethodLine::new().a_add_docs("keys")
                     .text("  .keys();")
                     .details("-&gt; iter over keys")
                     .finish(&mut r.hm, &mut section);

    MethodLine::new().a_add_docs("values")      .text("  .values")
                     .a_add_docs("values_mut")  .text("<span>_mut</span>();")
                     .details("-&gt; iter over values")
                     .finish(&mut r.hm, &mut section);


    MethodLine::new().a_add_docs("is_empty")
                     .text("  .is_empty();")
                     .details("-&gt; bool")
                     .finish(&mut r.hm, &mut section);

    MethodLine::new().a_add_docs("contains_key")
                     .text("  .contains_key(k:Q);")
                     .details("-&gt; bool")
                     .finish(&mut r.hm, &mut section);

    // MANIPULATING
    section.add_section("Manipulate");

    MethodLine::new().a_add_docs("get")     .text("  .get")
                     .a_add_docs("get_mut") .text("<span>_mut</span>(k:&amp;Q);")
                     .details("-&gt; Option&lt;&amp;V&gt;, K:Borrow&lt;Q&gt;")
                     .finish(&mut r.hm, &mut section);

    MethodLine::new().a_add_docs("entry")
                     .text("  .entry(key);")
                     .details("in place manipulation")
                     .finish(&mut r.hm, &mut section);

    MethodLine::new().a_add_docs("drain")
                     .text("  .drain();")
                     .details("-&gt; iter that drains")
                     .finish(&mut r.hm, &mut section);

    MethodLine::new().a_add_docs("clear")
                     .text("  .clear();")
                     .finish(&mut r.hm, &mut section);

    MethodLine::new().a_add_docs_use_id("extend", "hm.extend")
                     .text("  .extend(iter : &lt;Item=(")
                     .a_add_docs_use_id("extend-1", "hm.extend-1")
                     .text("<span>&amp;</span>K,<span>&amp;</span>V)&gt;);")
                     .finish(&mut r.hm, &mut section);

    MethodLine::new().a_add_docs("insert")
                     .text("  .insert(k,v);")
                     .details("-&gt; Option&lt;&amp;V&gt;, None on success.")
                     .finish(&mut r.hm, &mut section);

    MethodLine::new().a_add_docs("remove")
                     .text("  .remove(k:&amp;Q);")
                     .details("-&gt; Option&lt;&amp;V&gt;")
                     .finish(&mut r.hm, &mut section);

    MethodLine::new().a_add_docs("from_iter")
                     .text("  .from_iter(iter : &lt;Item=(K,V)&gt;);")
                     .details("-&gt; HashMap")
                     .finish(&mut r.hm, &mut section);

    section.add_section("Manage");


    MethodLine::new().a_add_docs("capacity")
                     .text("  .capacity();")
                     .finish(&mut r.hm, &mut section);

    MethodLine::new().a_add_docs("reserve")
                     .text("  .reserve(additional);")
                     .finish(&mut r.hm, &mut section);

    MethodLine::new().a_add_docs("shrink_to_fit")
                     .text("  .shrink_to_fit();")
                     .finish(&mut r.hm, &mut section);


    MethodLine::new().a_add_docs_use_id("clone_from", "hm.clone_from")
                     .text("  .clone_from(source);")
                     .details("overrides self")
                     .finish(&mut r.hm, &mut section);


    // COMPARISION
    section.add_section("Comparision");


    // cmp
    MethodLine::new().a_add_docs("eq") .text("  .eq() ")
                     .a_add_docs("ne") .text(".ne();")
                     .details("T: PartialEq")
                     .finish(&mut r.hm, &mut section);




    section.add_section("Special Hasher");

    MethodLine::new().a_add_docs("with_hasher")
                     .text("let hm = HashMap::with_hasher(b);")
                     .finish(&mut r.hm, &mut section);

    MethodLine::new().a_add_docs("with_capacity_and_hasher")
                     .text("       = HashMap::with_capacity_and_hasher(b);")
                     .finish(&mut r.hm, &mut section);

    MethodLine::new().a_add_docs("hasher")
                     .text("hm.hasher(b);")
                     .details("-&gt; &amp;BuildHasher")
                     .finish(&mut r.hm, &mut section);

    // TRAITS
    section.add_section("Traits");


    MethodLine::new()
        .a("trait.Clone")                       .text("Clone") .br()
        .a_select_add_docs(Doc::Impl("clone".into())).span("clone() clone_from() |") .br()
        .a("trait.PartialEq")                   .text("PartialEq") .br()
        .a_select_add_docs(Doc::Impl("eq".into())).span("eq() ne() |") .br()
        .a("trait.Eq")                          .text("Eq |") .br()
        .a("trait.Debug")                       .text("Debug") .br()
        .a_select_add_docs(Doc::Impl("fmt".into()))   .span("fmt() |") .br()
        .a("trait.Default")                       .text("Default") .br()
        .a_select_add_docs(Doc::Impl("default".into()))   .span("default() |") .br()
        .a("trait.Index")                       .text("Index") .br()
        .a_select_add_docs(Doc::Impl("index".into()))   .span("index() |") .br()
        .a("trait.IntoIterator")                .text("IntoIterator") .br()
        .a_select_add_docs(Doc::Nav("into_iter".into(), "pr".into(), "pnnnnn".into())).span("into_iter() |") .br()
        .a("trait.FromIterator")                .text("FromIterator") .br()
        .a_select_add_docs(Doc::Impl("from_iter".into())).span("from_iter() |") .br()
        .a("trait.Extend")                .text("Extend") .br()
        .a_select_add_docs(Doc::LastImpl("extend".into())).span("extend() |") .br()
        .finish(&mut r.hm, &mut section);



}