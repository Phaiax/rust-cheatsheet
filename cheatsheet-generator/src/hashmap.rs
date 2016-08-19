use ::{a1,a0,References,Group,sel};

pub fn make(r : &mut References, vector_box : &mut Group) {

   // Refernce: add_method_line(id, methodname, format, details)

    r.hm.add_doc_by_element("hm.hm", sel("section#main div.docblock"));
    vector_box.add_line_customdoc("<code>use std::collections::HashMap;</code>", "");

    // new
    vector_box.add_method_line("hm.new", "new", Some("let mut foo: HashMap&lt;K, V&gt; = HashMap::new();"), "")
              .doc(&mut r.hm);
    // with_capacity
    vector_box.add_method_line("hm.with_capacity", "with_capacity",
                               Some("            = HashMap::with_capacity();"),
                               "K: Eq, Hash").doc(&mut r.hm);

    vector_box.add_method_line("hm.clone", "clone",
                               Some("            = other.clone();"),
                               "if V,K:Clone").doc(&mut r.hm);

    // ACCESSING
    vector_box.add_section("Access");

    // v[]
    vector_box.add_method_line("hm.index", "index",
                               Some("foo[key];"), "").doc(&mut r.hm);;
    //r.hm.add_doc_by_element_range("hm.elementaccess", sel("#indexing"), sel("#slicing"));

    vector_box.add_method_line("hm.len", "len",
                               Some("foo.len();"),
                               "").doc(&mut r.hm);

    vector_box.add_method_line("hm.iter", "iter",
                               Some(&format!("  .iter{}<span>_mut</span>();",
                                a1("hm.iter_mut"),
                                )),
                               "-&gt; iter over (&amp;K, &amp;<span>mut </span>V)").doc(&mut r.hm);
    r.hm.add_doc_for_method("hm.iter_mut", "iter_mut");

    vector_box.add_method_line("hm.into_iter", "into_iter",
                               Some("  .into_iter();"),
                               "-&gt; Iter<span>Mut</span>").doc(&mut r.hm);

    vector_box.add_method_line("hm.keys", "keys",
                               Some("  .keys();"),
                               "-&gt; iter over keys").doc(&mut r.hm);

    vector_box.add_method_line("hm.values", "values",
                               Some(&format!("  .values{}<span>_mut</span>();",
                                a1("hm.values_mut"),
                                )),
                               "-&gt; iter over values").doc(&mut r.hm);
    r.hm.add_doc_for_method("hm.values_mut", "values_mut");


    vector_box.add_method_line("hm.is_empty", "is_empty",
                               Some("  .is_empty();"),
                               "-&gt; bool").doc(&mut r.hm);

    vector_box.add_method_line("hm.contains_key", "contains_key",
                               Some("  .contains_key(k:Q);"),
                               "-&gt; bool").doc(&mut r.hm);

    // MANIPULATING
    vector_box.add_section("Manipulate");

    vector_box.add_method_line("hm.get", "get",
                               Some(&format!("  .get{}<span>_mut</span>(k:&amp;Q);",
                                a1("hm.get_mut"),
                                )),
                               "-&gt; Option&lt;&amp;V&gt;, K:Borrow&lt;Q&gt;").doc(&mut r.hm);
    r.hm.add_doc_for_method("hm.get_mut", "get_mut");

    vector_box.add_method_line("hm.entry", "entry",
                               Some("  .entry(key);"),
                               "in place manipulation").doc(&mut r.hm);

    vector_box.add_method_line("hm.drain", "drain",
                               Some("  .drain();"),
                               "-&gt; iter that drains").doc(&mut r.hm);

    vector_box.add_method_line("hm.clear", "clear",
                               Some("  .clear();"),
                               "").doc(&mut r.hm);

    vector_box.add_method_line("hm.extend", "extend",
                               Some(&format!("  .extend(iter : &lt;Item=({}<span>&amp;</span>K,<span>&amp;</span>V)&gt;);",
                                a1("hm.extendmut"),
                                )),
                               "").doc(&mut r.hm);
    r.hm.add_doc_for_method("hm.extendmut", "extend-1");

    vector_box.add_method_line("hm.insert", "insert",
                               Some("  .insert(k,v);"),
                               "-&gt; Option&lt;&amp;V&gt;, None on success.").doc(&mut r.hm);

    vector_box.add_method_line("hm.remove", "remove",
                               Some("  .remove(k:&amp;Q);"),
                               "-&gt; Option&lt;&amp;V&gt;").doc(&mut r.hm);

    vector_box.add_method_line("hm.from_iter", "from_iter",
                               Some("  .from_iter(iter : &lt;Item=(K,V)&gt;);"),
                               "-&gt; HashMap").doc(&mut r.hm);

    vector_box.add_section("Manage");


    vector_box.add_method_line("hm.capacity", "capacity",
                               Some("  .capacity();"),
                               "").doc(&mut r.hm);

    vector_box.add_method_line("hm.reserve", "reserve",
                               Some("  .reserve(additional);"),
                               "").doc(&mut r.hm);

    vector_box.add_method_line("hm.shrink_to_fit", "shrink_to_fit",
                               Some("  .shrink_to_fit();"),
                               "").doc(&mut r.hm);


    vector_box.add_method_line("hm.clone_from", "clone_from",
                               Some("  .clone_from(source);"),
                               "overrides self").doc(&mut r.hm);


    // COMPARISION
    vector_box.add_section("Comparision");


    // cmp
    vector_box.add_method_line("hm.eq", "eq",
                               Some(&format!("  .eq() {}.ne();",
                                a1("hm.ne"),)),
                               "T: PartialEq")
                               .doc(&mut r.hm);
    r.hm.add_doc_for_method("hm.ne", "ne");




    vector_box.add_section("Special Hasher");

    vector_box.add_method_line("hm.with_hasher", "with_hasher",
                               Some("let hm = HashMap::with_hasher(b);"),
                               "").doc(&mut r.hm);
    vector_box.add_method_line("hm.with_capacity_and_hasher", "with_capacity_and_hasher",
                               Some("       = HashMap::with_capacity_and_hasher(b);"),
                               "").doc(&mut r.hm);

    vector_box.add_method_line("hm.hasher", "hasher",
                               Some("hm.hasher(b);"),
                               "-&gt; &amp;BuildHasher").doc(&mut r.hm);

    // TRAITS
    vector_box.add_section("Traits");

    vector_box.add_line_customdoc(&format!("
        {}<code>Clone {}<span>+</span></code>,
        {}<code>Index</code>,
        {}<code>FromIterator</code>,
        {}<code>IntoIterator</code>,
        {}<code>Extend {}<span>+</span></code>,
        {}<code>PartialEq</code>,
        {}<code>Eq</code>,
        {}<code>Default</code>,
        {}<code>Debug (if K,V:Debug)</code>,
        </a>",
        a0("hm.clone"),
        a1("hm.clone_from"),
        a1("hm.index"),
        a1("trait.FromIterator"),
        a1("trait.IntoIterator"),
        a1("hm.extend"),
        a1("hm.extend-1"),
        a1("trait.PartialEq"),
        a1("trait.Eq"),
        a1("hm.Default"),
        a1("hm.Debug"),
        ), "" );

    r.hm.add_doc_for_method("hm.Default", "default");
    r.hm.add_doc_for_method("hm.Debug", "fmt");

}