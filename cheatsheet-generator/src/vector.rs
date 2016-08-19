
use ::{a1,a0,References,Group,sel};

pub fn make(r : &mut References, vector_box : &mut Group) {

   // Refernce: add_method_line(id, methodname, format, details)

    // Intentionally missing functions for Vec
    // unsafe fn get_unchecked(&self, index: usize) -> &T
    // unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T
    // fn as_ptr(&self) -> *const T
    // fn as_mut_ptr(&mut self) -> *mut T
    // unsafe fn set_len(&mut self, len: usize)
    // unsafe fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Vec<T>

    // header
    r.vector.add_doc_by_element("vec.vec", sel("section#main div.docblock"));

    // new
    vector_box.add_method_line("vec.new", "new", Some("let mut vec: Vec&lt;T&gt; = Vec::new();"), "")
              .doc(&mut r.vector);
    // with_capacity
    vector_box.add_method_line("vec.with_capacity", "with_capacity",
                               Some("            = Vec::with_capacity();"),
                               "").doc(&mut r.vector);

    // vec![]
    vector_box.add_method_line("vec.initmacro", "",
                               Some("            = vec![];"), "");
    r.vector_macro.add_doc_by_element("vec.initmacro", sel("section#main"));

    // boxed array -> Vec
    vector_box.add_method_line("boxedvec.into_vec", "into_vec",
                               Some("            = boxedarray.into_vec()"), "")
                                .doc(&mut r.vector);
    // From
    vector_box.add_method_line("vec.from", "from-1",
                               Some(&format!("            = Vec::from(slice|{}str|{}VecDeque|{}CString)",
                                a1("vec.from-str"),
                                a1("vec.from-vecdeque"),
                                a1("vec.from-cstring"),
                                )), "").doc(&mut r.vector);
    r.vector.add_doc_for_method("vec.from-str", "from-2");
    r.vector.add_doc_for_method("vec.from-vecdeque", "from-3");
    r.vector.add_doc_for_method("vec.from-cstring", "from-4");


    // clone
    vector_box.add_method_line("vec.clone", "clone",
                               Some("            = othervec.clone();"), "if T:Clone")
                               .doc(&mut r.vector);


    // ACCESSING
    vector_box.add_section("Accessing");

    // v[]
    vector_box.add_method_line("vec.elementaccess", "",
                               Some("vec[3];"), "vec[1..3], vec[..3], vec[3..], vec[..]; vec[2] = a;");
    r.vector.add_doc_by_element_range("vec.elementaccess", sel("#indexing"), sel("#slicing"));


    // len
    vector_box.add_method_line("vec.len", "len", Some("vec.len();"), "").doc(&mut r.vector);

    // is_empty
    vector_box.add_method_line("vec.is_empty", "is_empty", None, "").doc(&mut r.vector);

    // first_mut, last_mut
    vector_box.add_method_line("vec.first", "first",
                               Some(&format!("  .first{}<span>_mut</span>(); {}.last{}<span>_mut</span>();",
                                    a1("vec.first_mut"),
                                    a1("vec.last"),
                                    a1("vec.last_mut")
                                    )), "-&gt; Option").doc(&mut r.vector);
    r.vector.add_doc_for_method("vec.first_mut", "first_mut");
    r.vector.add_doc_for_method("vec.last", "last");
    r.vector.add_doc_for_method("vec.last_mut", "last_mut");

    // get_mut
    vector_box.add_method_line("vec.get", "get",
                               Some(&format!("  .get{}<span>_mut</span>(index);",
                                    a1("vec.get_mut"),
                                    )), "-&gt; Option").doc(&mut r.vector);
    r.vector.add_doc_for_method("vec.get_mut", "get_mut");


    // contains
    vector_box.add_method_line("vec.contains", "contains",
                               Some("  .contains(needle);"), "-&gt; bool")
                               .doc(&mut r.vector);

    // find
    vector_box.add_method_line("iter.find", "",
                               Some("  .iter().find(|&amp;T| -> bool);"), "-&gt; Option");
    r.iter.add_doc_for_method("iter.find", "find");

    // binary_search
    vector_box.add_method_line("vec.binary_search", "binary_search",
                               Some("  .binary_search(x:&amp;T);"), "-&gt; Result&lt;usize, usize&gt;<br>Ok(i): pos, Err(i): pos for insertion")
                               .doc(&mut r.vector);

    // binary_search_by
    vector_box.add_method_line("vec.binary_search_by", "binary_search_by",
                               Some("  .binary_search_by(|&amp;T|->Ordering);"), "")
                               .doc(&mut r.vector);

    // binary_search_by_key
    vector_box.add_method_line("vec.binary_search_by_key", "binary_search_by_key",
                               Some("  .binary_search_by_key(Key, |&amp;T|->Key);"), "")
                               .doc(&mut r.vector);

    // ends_with
    vector_box.add_method_line("vec.ends_with", "ends_with",
                               Some("  .ends_with(needle);"), "")
                               .doc(&mut r.vector);

    // starts_with
    vector_box.add_method_line("vec.starts_with", "starts_with",
                               Some("  .starts_with(needle);"), "")
                               .doc(&mut r.vector);

    // ADDING
    vector_box.add_section("Adding");

    // push
    vector_box.add_method_line("vec.push", "push",
                               Some("  .push(3);"), "to end")
                               .doc(&mut r.vector);

    // insert
    vector_box.add_method_line("vec.insert", "insert",
                               Some("  .insert(index, element);"), "")
                               .doc(&mut r.vector);

    // extend
    vector_box.add_method_line("vec.extend", "extend",
                               Some("  .extend(iterable);"), "")
                               .doc(&mut r.vector);

    // extend_from_slice
    vector_box.add_method_line("vec.extend_from_slice", "extend_from_slice",
                               Some("  .extend_from_slice(&amp;[T]);"), "")
                               .doc(&mut r.vector);

    // append
    vector_box.add_method_line("vec.append", "append",
                                Some("  .append(other : Vec);"), "drains other")
                                .doc(&mut r.vector);

    // clone_from
    vector_box.add_method_line("vec.clone_from", "clone_from",
                               Some("  .clone_from(&amp;Vec&lt;T&gt;);"), "overrides self, if T:Clone")
                               .doc(&mut r.vector);

    // clone_from_slice
    vector_box.add_method_line("vec.clone_from_slice", "clone_from_slice",
                               Some("  .clone_from_slice(&amp;[T]);"), "if T:Clone")
                               .doc(&mut r.vector);

    // copy_from_slice
    vector_box.add_method_line("vec.copy_from_slice", "copy_from_slice",
                               Some("  .copy_from_slice(&amp;[T]);"), "if T:Copy (use memcpy)")
                               .doc(&mut r.vector);

    // REMOVING
    vector_box.add_section("Removing");

    // pop
    vector_box.add_method_line("vec.pop", "pop",
                               Some("  .pop();"), "removes last -> Option")
                               .doc(&mut r.vector);

    // remove
    vector_box.add_method_line("vec.remove", "remove",
                               Some("  .remove(index);"), "-&gt; el, shifts left")
                               .doc(&mut r.vector);

    // swap_remove
    vector_box.add_method_line("vec.swap_remove", "swap_remove",
                               Some("  .swap_remove(index);"), "-&gt; el, fills with last")
                               .doc(&mut r.vector);

    // truncate
    vector_box.add_method_line("vec.truncate", "truncate",
                               Some("  .truncate(i);"), "cut until .len() = i")
                               .doc(&mut r.vector);

    // drain
    vector_box.add_method_line("vec.drain", "drain",
                               Some("  .drain(range);"), "-&gt; iter that drains")
                               .doc(&mut r.vector);

    // clear
    vector_box.add_method_line("vec.clear", "clear",
                               Some("  .clear();"), "")
                               .doc(&mut r.vector);

    // retain
    vector_box.add_method_line("vec.retain", "retain",
                               Some("  .retain(|i| -&gt; bool);"), "in place")
                               .doc(&mut r.vector);

    // dedup
    vector_box.add_method_line("vec.dedup", "dedup",
                               Some("  .dedup();"), "removes duplicates (if T:PartialEq)")
                               .doc(&mut r.vector);

    // MANIPULATING
    vector_box.add_section("Manipulating");

    // sort
    vector_box.add_method_line("vec.sort", "sort",
                               Some("  .sort();"), "in place")
                               .doc(&mut r.vector);

    // sort_by
    vector_box.add_method_line("vec.sort_by", "sort_by",
                               Some("  .sort_by(|&amp;T|->Ordering);"), "in place")
                               .doc(&mut r.vector);

    // sort_by_key
    vector_box.add_method_line("vec.sort_by_key", "sort_by_key",
                               Some("  .sort_by_key(|&amp;T|->Key);"), "Key:Ordering")
                               .doc(&mut r.vector);

    // reverse
    vector_box.add_method_line("vec.reverse", "reverse",
                               Some("  .reverse();"), "in place")
                               .doc(&mut r.vector);

    // swap
    vector_box.add_method_line("vec.swap", "swap",
                               Some("  .swap(index1, index2);"), "")
                               .doc(&mut r.vector);


    // TRANSFORMING
    vector_box.add_section("Transforming (Iter, as_, to_)");

    // iter
    vector_box.add_method_line("vec.iter", "iter",
                               Some(&format!("  .iter{}<span>_mut</span>();",
                                a1("vec.iter_mut"),
                                )), "<div title=\"borrows value\">-&gt;&<span>mut </span>T, keeps vector</div>")
                               .doc(&mut r.vector);
    r.vector.add_doc_for_method("vec.iter_mut", "iter_mut");

    // into_iter
    vector_box.add_method_line("vec.into_iter", "into_iter",
                               Some("  .into_iter();"), "<div title=\"transfers ownership\">-&gt;T, consumes vector</div>")
                               .doc(&mut r.vector);

    // chunks
    vector_box.add_method_line("vec.chunks", "chunks",
                               Some(&format!("  .chunks{}<span>_mut</span>(cnk_sz);",
                                a1("vec.chunks_mut"),
                                )), "-&gt; iter over a non overlapping slice at a time")
                               .doc(&mut r.vector);
    r.vector.add_doc_for_method("vec.chunks_mut", "chunks_mut");

    // windows
    vector_box.add_method_line("vec.windows", "windows",
                               Some("  .windows(wnd_sz);"), "-&gt; iter over an overlapping slice at a time")
                               .doc(&mut r.vector);
    vector_box.add_hr();

    // into_boxed_slice
    vector_box.add_method_line("vec.into_boxed_slice", "into_boxed_slice",
                               Some("  .into_boxed_slice();"), "-&gt; Box&lt;T&gt;")
                               .doc(&mut r.vector);

    // as_ref
    vector_box.add_method_line("vec.as_ref", "as_ref",
                               Some("  .as_ref();"), "-&gt; &amp;[T] or &amp;Vec&lt;T&gt;")
                               .doc(&mut r.vector);

    // to_vec
    vector_box.add_method_line("vec.to_vec", "to_vec",
                               Some("  .to_vec();"), "like clone(), if T:Clone")
                               .doc(&mut r.vector);

    // as_slice
    vector_box.add_method_line("vec.as_slice", "as_slice",
                               Some(&format!("  .as{}<span>_mut</span>{}_slice();",
                                a1("vec.as_mut_slice"),
                                a1("vec.as_slice"))), "-&gt; &amp;<span>mut</span>[T]")
                               .doc(&mut r.vector);
    r.vector.add_doc_for_method("vec.as_mut_slice", "as_mut_slice");


    // MEMORY
    vector_box.add_section("Memory");

    // capacity
    vector_box.add_method_line("vec.capacity", "capacity",
                               Some("  .capacity();"), "")
                               .doc(&mut r.vector);

    // reserve
    vector_box.add_method_line("vec.reserve", "reserve",
                               Some("  .reserve(100);"), "in addition to .len() or more")
                               .doc(&mut r.vector);

    // reserve_exact
    vector_box.add_method_line("vec.reserve_exact", "reserve_exact",
                               Some("  .reserve_exact(100);"), "in addition to .len()")
                               .doc(&mut r.vector);

    // shrink_to_fit
    vector_box.add_method_line("vec.shrink_to_fit", "shrink_to_fit",
                               Some("  .shrink_to_fit();"), "")
                               .doc(&mut r.vector);

    // MEMORY
    vector_box.add_section("Split");


    // split_at
    vector_box.add_method_line("vec.split_at", "split_at",
                               Some(&format!("  .split_at{}<span>_mut</span>(mid);",
                                a1("vec.split_at_mut"),
                                )), "-&gt; (p1, p2), [mid] in 2nd part")
                               .doc(&mut r.vector);
    r.vector.add_doc_for_method("vec.split_at_mut", "split_at_mut");


    vector_box.add_method_line("vec.split", "split",
                               Some(&format!("  .split{}<span>_mut</span>(|&amp;T| -&gt; bool);",
                                    a1("vec.split_mut"),
                                    )), "").doc(&mut r.vector);
    r.vector.add_doc_for_method("vec.split_mut", "split_mut");


    vector_box.add_method_line("vec.splitn", "splitn",
                               Some(&format!("  .splitn{}<span>_mut</span>(n, |&amp;T| -&gt; bool); {}.rsplitn{}<span>_mut</span>(_);",
                                    a1("vec.splitn_mut"),
                                    a1("vec.rsplitn"),
                                    a1("vec.rsplitn_mut"),
                                    )), "-&gt; iter over <span>mutable</span> subslices,<br> seperated by ||-&gt;true, <span>at most n times</span>").doc(&mut r.vector);

    r.vector.add_doc_for_method("vec.splitn_mut", "splitn_mut");
    r.vector.add_doc_for_method("vec.rsplitn", "rsplitn");
    r.vector.add_doc_for_method("vec.rsplitn_mut", "rsplitn_mut");


    vector_box.add_method_line("vec.split_off", "split_off",
                               Some("  .split_off(mid);"), "-&gt; Vec; [mid] in 2nd part")
                               .doc(&mut r.vector);

    // COMPARISION
    vector_box.add_section("Comparision");

    // cmp
    vector_box.add_method_line("vec.cmp", "cmp",
                               Some(&format!("  .cmp() {}.eq() {}.ne();",
                                a1("vec.eq"),
                                a1("vec.ne"),)),
                               "T: PartialEq")
                               .doc(&mut r.vector);
    r.vector.add_doc_for_method("vec.eq", "eq");
    r.vector.add_doc_for_method("vec.ne", "ne");

    // lt
    vector_box.add_method_line("vec.lt", "lt",
                               Some(&format!("  .lt() {}.le() {}.gt() {}.ge();",
                                a1("vec.le"),
                                a1("vec.gt"),
                                a1("vec.ge"))),
                                "if T:PartialOrd")
                               .doc(&mut r.vector);
    r.vector.add_doc_for_method("vec.le", "le");
    r.vector.add_doc_for_method("vec.gt", "gt");
    r.vector.add_doc_for_method("vec.ge", "ge");

    vector_box.add_hr();

    // hash
    vector_box.add_method_line("vec.hash", "hash",
                               Some("  .hash(state: Hasher)"), "if T:Hash")
                               .doc(&mut r.vector);

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
        a1("trait.FromIterator"),
        a1("trait.IntoIterator"),
        a1("vec.extend"),
        a1("trait.PartialEq"),
        a1("trait.PartialOrd"),
        a1("trait.Eq"),
        a1("trait.Ord"),
        a1("vec.Drop"),
        a1("vec.Default"),
        a1("vec.Debug"),
        a1("vec.AsRef"),
        a1("vec.AsMut"),
        a1("vec.From"),
        a1("trait.Write"),
        ), "" );

    r.vector.add_doc_for_method("vec.BinaryHeap-from", "from");
    r.vector.add_doc_for_method("vec.borrow", "borrow");
    r.vector.add_doc_for_method("vec.borrow_mut", "borrow_mut");
    r.vector.add_doc_for_method("vec.clone", "clone");
    r.vector.add_doc_for_method("vec.clone_from", "clone_from");
    r.vector.add_doc_for_method("vec.hash", "hash");
    r.vector.add_doc_for_method("vec.hash_slice", "hash_slice");
    r.vector.add_doc_for_method("vec.index", "index");
    r.vector.add_doc_for_method("vec.index_mut", "index_mut");
    r.vector.add_doc_for_method("vec.deref", "deref");
    r.vector.add_doc_for_method("vec.deref_mut", "deref_mut");
    // FromIter (in main.rs)
    // IntoIter (in main.rs)
    r.vector.add_doc_for_method("vec.extend", "extend");
    // PartialOrd (in main.rs)
    // PartialEq (in main.rs)
    // Ord (in main.rs)
    // Eq (in main.rs)
    r.vector.add_doc_for_method("vec.Drop", "drop");
    r.vector.add_doc_for_method("vec.Default", "default");
    r.vector.add_doc_for_method("vec.Debug", "fmt");
    r.vector.add_doc_for_method("vec.AsRef", "as_ref");
    r.vector.add_doc_for_method("vec.AsMut", "as_mut");
    r.vector.add_doc_for_method("vec.From", "from-1");
    // Write (in main.rs)

}