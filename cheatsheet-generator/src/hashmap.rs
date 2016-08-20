use ::{References,Group,sel,MethodLine};

pub fn make(r : &mut References, mut vector_box : &mut Group) {

   // Refernce: add_method_line(id, methodname, format, details)

    r.hm.add_doc_by_element("hm.hm", sel("section#main div.docblock"));
    vector_box.add_line_customdoc("<code>use std::collections::HashMap;</code>", "");

    // new
    MethodLine::new().a_add_docs("new")
                     .text( "let mut foo: HashMap&lt;K, V&gt; = HashMap::new();")
                     .finish(&mut r.hm, &mut vector_box);

    // with_capacity
    MethodLine::new().a_add_docs("with_capacity")
                     .text("            = HashMap::with_capacity();")
                     .details("K: Eq, Hash")
                     .finish(&mut r.hm, &mut vector_box);

    MethodLine::new().a_add_docs_use_id("clone", "hm.clone")
                     .text("            = other.clone();")
                     .details("if V,K:Clone")
                     .finish(&mut r.hm, &mut vector_box);

    // ACCESSING
    vector_box.add_section("Access");

    // v[]
    MethodLine::new().a_add_docs_use_id("index", "hm.index")
                     .text("foo[key];")
                     .finish(&mut r.hm, &mut vector_box);;
    //r.hm.add_doc_by_element_range("hm.elementaccess", sel("#indexing") sel("#slicing"));

    MethodLine::new().a_add_docs("len")
                     .text("foo.len();")
                     .finish(&mut r.hm, &mut vector_box);

    MethodLine::new().a_add_docs("iter")        .text("  .iter")
                     .a_add_docs("iter_mut")    .text("<span>_mut</span>();")
                     .details("-&gt; iter over (&amp;K, &amp;<span>mut </span>V)")
                     .finish(&mut r.hm, &mut vector_box);

    MethodLine::new().a_add_docs("into_iter")
                     .text("  .into_iter();")
                     .details("-&gt; Iter<span>Mut</span>")
                     .finish(&mut r.hm, &mut vector_box);

    MethodLine::new().a_add_docs("keys")
                     .text("  .keys();")
                     .details("-&gt; iter over keys")
                     .finish(&mut r.hm, &mut vector_box);

    MethodLine::new().a_add_docs("values")      .text("  .values")
                     .a_add_docs("values_mut")  .text("<span>_mut</span>();")
                     .details("-&gt; iter over values")
                     .finish(&mut r.hm, &mut vector_box);


    MethodLine::new().a_add_docs("is_empty")
                     .text("  .is_empty();")
                     .details("-&gt; bool")
                     .finish(&mut r.hm, &mut vector_box);

    MethodLine::new().a_add_docs("contains_key")
                     .text("  .contains_key(k:Q);")
                     .details("-&gt; bool")
                     .finish(&mut r.hm, &mut vector_box);

    // MANIPULATING
    vector_box.add_section("Manipulate");

    MethodLine::new().a_add_docs("get")     .text("  .get")
                     .a_add_docs("get_mut") .text("<span>_mut</span>(k:&amp;Q);")
                     .details("-&gt; Option&lt;&amp;V&gt;, K:Borrow&lt;Q&gt;")
                     .finish(&mut r.hm, &mut vector_box);

    MethodLine::new().a_add_docs("entry")
                     .text("  .entry(key);")
                     .details("in place manipulation")
                     .finish(&mut r.hm, &mut vector_box);

    MethodLine::new().a_add_docs("drain")
                     .text("  .drain();")
                     .details("-&gt; iter that drains")
                     .finish(&mut r.hm, &mut vector_box);

    MethodLine::new().a_add_docs("clear")
                     .text("  .clear();")
                     .finish(&mut r.hm, &mut vector_box);

    MethodLine::new().a_add_docs_use_id("extend", "hm.extend")
                     .text("  .extend(iter : &lt;Item=(")
                     .a_add_docs_use_id("extend-1", "hm.extend-1")
                     .text("<span>&amp;</span>K,<span>&amp;</span>V)&gt;);")
                     .finish(&mut r.hm, &mut vector_box);

    MethodLine::new().a_add_docs("insert")
                     .text("  .insert(k,v);")
                     .details("-&gt; Option&lt;&amp;V&gt;, None on success.")
                     .finish(&mut r.hm, &mut vector_box);

    MethodLine::new().a_add_docs("remove")
                     .text("  .remove(k:&amp;Q);")
                     .details("-&gt; Option&lt;&amp;V&gt;")
                     .finish(&mut r.hm, &mut vector_box);

    MethodLine::new().a_add_docs("from_iter")
                     .text("  .from_iter(iter : &lt;Item=(K,V)&gt;);")
                     .details("-&gt; HashMap")
                     .finish(&mut r.hm, &mut vector_box);

    vector_box.add_section("Manage");


    MethodLine::new().a_add_docs("capacity")
                     .text("  .capacity();")
                     .finish(&mut r.hm, &mut vector_box);

    MethodLine::new().a_add_docs("reserve")
                     .text("  .reserve(additional);")
                     .finish(&mut r.hm, &mut vector_box);

    MethodLine::new().a_add_docs("shrink_to_fit")
                     .text("  .shrink_to_fit();")
                     .finish(&mut r.hm, &mut vector_box);


    MethodLine::new().a_add_docs_use_id("clone_from", "hm.clone_from")
                     .text("  .clone_from(source);")
                     .details("overrides self")
                     .finish(&mut r.hm, &mut vector_box);


    // COMPARISION
    vector_box.add_section("Comparision");


    // cmp
    MethodLine::new().a_add_docs("eq") .text("  .eq() ")
                     .a_add_docs("ne") .text(".ne();")
                     .details("T: PartialEq")
                     .finish(&mut r.hm, &mut vector_box);




    vector_box.add_section("Special Hasher");

    MethodLine::new().a_add_docs("with_hasher")
                     .text("let hm = HashMap::with_hasher(b);")
                     .finish(&mut r.hm, &mut vector_box);

    MethodLine::new().a_add_docs("with_capacity_and_hasher")
                     .text("       = HashMap::with_capacity_and_hasher(b);")
                     .finish(&mut r.hm, &mut vector_box);

    MethodLine::new().a_add_docs("hasher")
                     .text("hm.hasher(b);")
                     .details("-&gt; &amp;BuildHasher")
                     .finish(&mut r.hm, &mut vector_box);

    // TRAITS
    vector_box.add_section("Traits");

    MethodLine::new()
        .a("hm.clone")        .text("<code>Clone ")
        .a("hm.clone_from")    .text("<span>+</span></code>,")
        .a("hm.index")         .text("<code>Index</code>,")
        .a("trait.FromIterator") .text("<code>FromIterator</code>,")
        .a("trait.IntoIterator") .text("<code>IntoIterator</code>,")
        .a("hm.extend")        .text("<code>Extend ")
        .a("hm.extend-1")      .text("<span>+</span></code>,")
        .a("trait.PartialEq")  .text("<code>PartialEq</code>,")
        .a("trait.Eq")         .text("<code>Eq</code>,")
        .a_add_docs("default") .text("<code>Default</code>,")
        .a_add_docs("fmt")     .text("<code>Debug (if K,V:Debug)</code>")
                     .finish(&mut r.hm, &mut vector_box);


}