
use ::{References,Group,sel,MethodLine};

pub fn make(r : &mut References, mut vector_box : &mut Group) {

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
    MethodLine::new().a_add_docs("new")
                     .text("let mut vec: Vec&lt;T&gt; = Vec::new();")
                     .finish(&mut r.vector, &mut vector_box);

    // with_capacity
    MethodLine::new().a_add_docs("with_capacity")
                .text("            = Vec::with_capacity();")
                .finish(&mut r.vector, &mut vector_box);

    // vec![]
    MethodLine::new().a("vec.initmacro")
                     .text("            = vec![];")
                     .finish(&mut r.vector, &mut vector_box);
    r.vector_macro.add_doc_by_element("vec.initmacro", sel("section#main"));

    // boxed array -> Vec
    MethodLine::new().a_add_docs("into_vec")
                     .text("            = boxedarray.into_vec()")
                     .finish(&mut r.vector, &mut vector_box);

    // From
    MethodLine::new().a_add_docs("from-1") .text("            = Vec::from(slice|")
                     .a_add_docs("from-2") .text("str|")
                     .a_add_docs("from-3") .text("VecDeque|")
                     .a_add_docs("from-4") .text("CString)")
                     .finish(&mut r.vector, &mut vector_box);

    // clone
    MethodLine::new().a_add_docs("clone")
                     .text("            = othervec.clone();")
                     .details("if T:Clone")
                     .finish(&mut r.vector, &mut vector_box);


    // ACCESSING
    vector_box.add_section("Accessing");

    // v[]
    MethodLine::new().a("vec.elementaccess")
                     .text("vec[3];")
                     .details("vec[1..3], vec[..3], vec[3..], vec[..]; vec[2] = a;")
                     .finish(&mut r.vector, &mut vector_box);
    r.vector.add_doc_by_element_range("vec.elementaccess", sel("#indexing"), sel("#slicing"));


    // len
    MethodLine::new().a_add_docs("len")
                     .text("vec.len();")
                     .finish(&mut r.vector, &mut vector_box);

    // is_empty
    MethodLine::new().single_method_with_doc("is_empty")
                     .finish(&mut r.vector, &mut vector_box);

    // first_mut, last_mut
    MethodLine::new().a_add_docs("first")       .text("  .first")
                                  .a_add_docs("first_mut")   .text("<span>_mut</span>(); ")
                                  .a_add_docs("last")        .text(".last")
                                  .a_add_docs("last_mut")    .text("<span>_mut</span>();")
                     .details("-&gt; Option")
                     .finish(&mut r.vector, &mut vector_box);

    // get_mut
    MethodLine::new().a_add_docs("get")     .text("  .get")
                                 .a_add_docs("get_mut")  .text("<span>_mut</span>(index);")
                    .details("-&gt; Option")
                    .finish(&mut r.vector, &mut vector_box);


    // contains
    MethodLine::new().a_add_docs("contains")
                     .text("  .contains(needle);")
                     .details("-&gt; bool")
                     .finish(&mut r.vector, &mut vector_box);

    // find
    MethodLine::new().a("iter.find")
                     .text("  .iter().find(|&amp;T| -> bool);")
                     .details("-&gt; Option")
                     .finish(&mut r.vector, &mut vector_box);
    r.iter.add_doc_for_method("iter.find", "find");

    // binary_search
    MethodLine::new().a_add_docs("binary_search")
                     .text("  .binary_search(x:&amp;T);")
                     .details("-&gt; Result&lt;usize, usize&gt;<br>Ok(i): pos, Err(i): pos for insertion")
                     .finish(&mut r.vector, &mut vector_box);

    // binary_search_by
    MethodLine::new().a_add_docs("binary_search_by")
                     .text("  .binary_search_by(|&amp;T|->Ordering);")
                     .finish(&mut r.vector, &mut vector_box);

    // binary_search_by_key
    MethodLine::new().a_add_docs("binary_search_by_key")
                     .text("  .binary_search_by_key(Key, |&amp;T|->Key);")
                     .finish(&mut r.vector, &mut vector_box);

    // ends_with
    MethodLine::new().a_add_docs("ends_with")
                     .text("  .ends_with(needle);")
                     .finish(&mut r.vector, &mut vector_box);

    // starts_with
    MethodLine::new().a_add_docs("starts_with")
                     .text("  .starts_with(needle);")
                     .finish(&mut r.vector, &mut vector_box);

    // ADDING
    vector_box.add_section("Adding");

    // push
    MethodLine::new().a_add_docs("push")
                     .text("  .push(3);")
                     .details("to end")
                     .finish(&mut r.vector, &mut vector_box);

    // insert
    MethodLine::new().a_add_docs("insert")
                     .text("  .insert(index, element);")
                     .finish(&mut r.vector, &mut vector_box);

    // extend
    MethodLine::new().a_add_docs("extend")
                     .text("  .extend(iterable);")
                     .finish(&mut r.vector, &mut vector_box);

    // extend_from_slice
    MethodLine::new().a_add_docs("extend_from_slice")
                     .text("  .extend_from_slice(&amp;[T]);")
                     .finish(&mut r.vector, &mut vector_box);

    // append
    MethodLine::new().a_add_docs("append")
                     .text("  .append(other : Vec);")
                     .details("drains other")
                     .finish(&mut r.vector, &mut vector_box);

    // clone_from
    MethodLine::new().a_add_docs("clone_from")
                     .text("  .clone_from(&amp;Vec&lt;T&gt;);")
                     .details("overrides self, if T:Clone")
                     .finish(&mut r.vector, &mut vector_box);

    // clone_from_slice
    MethodLine::new().a_add_docs("clone_from_slice")
                     .text("  .clone_from_slice(&amp;[T]);")
                     .details("if T:Clone")
                     .finish(&mut r.vector, &mut vector_box);

    // copy_from_slice
    MethodLine::new().a_add_docs("copy_from_slice")
                     .text("  .copy_from_slice(&amp;[T]);")
                     .details("if T:Copy (use memcpy)")
                     .finish(&mut r.vector, &mut vector_box);

    // REMOVING
    vector_box.add_section("Removing");

    // pop
    MethodLine::new().a_add_docs("pop")
                     .text("  .pop();")
                     .details("removes last -> Option")
                     .finish(&mut r.vector, &mut vector_box);

    // remove
    MethodLine::new().a_add_docs("remove")
                     .text("  .remove(index);")
                     .details("-&gt; el, shifts left")
                     .finish(&mut r.vector, &mut vector_box);

    // swap_remove
    MethodLine::new().a_add_docs("swap_remove")
                     .text("  .swap_remove(index);")
                     .details("-&gt; el, fills with last")
                     .finish(&mut r.vector, &mut vector_box);

    // truncate
    MethodLine::new().a_add_docs("truncate")
                     .text("  .truncate(i);")
                     .details("cut until .len() = i")
                     .finish(&mut r.vector, &mut vector_box);

    // drain
    MethodLine::new().a_add_docs("drain")
                     .text("  .drain(range);")
                     .details("-&gt; iter that drains")
                     .finish(&mut r.vector, &mut vector_box);

    // clear
    MethodLine::new().a_add_docs("clear")
                     .text("  .clear();")
                     .finish(&mut r.vector, &mut vector_box);

    // retain
    MethodLine::new().a_add_docs("retain")
                     .text("  .retain(|i| -&gt; bool);")
                     .details("in place")
                     .finish(&mut r.vector, &mut vector_box);

    // dedup
    MethodLine::new().a_add_docs("dedup")
                     .text("  .dedup();")
                     .details("removes duplicates (if T:PartialEq)")
                     .finish(&mut r.vector, &mut vector_box);

    // MANIPULATING
    vector_box.add_section("Manipulating");

    // sort
    MethodLine::new().a_add_docs("sort")
                     .text("  .sort();")
                     .details("in place")
                     .finish(&mut r.vector, &mut vector_box);

    // sort_by
    MethodLine::new().a_add_docs("sort_by")
                     .text("  .sort_by(|&amp;T|->Ordering);")
                     .details("in place")
                     .finish(&mut r.vector, &mut vector_box);

    // sort_by_key
    MethodLine::new().a_add_docs("sort_by_key")
                     .text("  .sort_by_key(|&amp;T|->Key);")
                     .details("Key:Ordering")
                     .finish(&mut r.vector, &mut vector_box);

    // reverse
    MethodLine::new().a_add_docs("reverse")
                     .text("  .reverse();")
                     .details("in place")
                     .finish(&mut r.vector, &mut vector_box);

    // swap
    MethodLine::new().a_add_docs("swap")
                     .text("  .swap(index1, index2);")
                     .finish(&mut r.vector, &mut vector_box);


    // TRANSFORMING
    vector_box.add_section("Transforming (Iter, as_, to_)");

    // iter
    MethodLine::new().a_add_docs("iter")     .text("  .iter")
                     .a_add_docs("iter_mut") .text("<span>_mut</span>();")
                     .details("<div title=\"borrows value\">-&gt;&<span>mut </span>T, keeps vector</div>")
                     .finish(&mut r.vector, &mut vector_box);

    // into_iter
    MethodLine::new().a_add_docs("into_iter")
                     .text("  .into_iter();")
                     .details("<div title=\"transfers ownership\">-&gt;T, consumes vector</div>")
                     .finish(&mut r.vector, &mut vector_box);

    // chunks
    MethodLine::new().a_add_docs("chunks")      .text("  .chunks")
                     .a_add_docs("chunks_mut")  .text("<span>_mut</span>(cnk_sz);")
                     .details("-&gt; iter over a non overlapping slice at a time")
                     .finish(&mut r.vector, &mut vector_box);

    // windows
    MethodLine::new().a_add_docs("windows")
                     .text("  .windows(wnd_sz);")
                     .details("-&gt; iter over an overlapping slice at a time")
                     .finish(&mut r.vector, &mut vector_box);
    vector_box.add_hr();

    // into_boxed_slice
    MethodLine::new().a_add_docs("into_boxed_slice")
                     .text("  .into_boxed_slice();")
                     .details("-&gt; Box&lt;T&gt;")
                     .finish(&mut r.vector, &mut vector_box);

    // as_ref
    MethodLine::new().a_add_docs("as_ref")
                     .text("  .as_ref();")
                     .details("-&gt; &amp;[T] or &amp;Vec&lt;T&gt;")
                     .finish(&mut r.vector, &mut vector_box);

    // to_vec
    MethodLine::new().a_add_docs("to_vec")
                     .text("  .to_vec();")
                     .details("like clone(), if T:Clone")
                     .finish(&mut r.vector, &mut vector_box);

    // as_slice
    MethodLine::new().a_add_docs("as_slice")     .text("  .as")
                     .a_add_docs("as_mut_slice") .text("<span>_mut</span>")
                                                 .text("_slice();")
                     .details("-&gt; &amp;<span>mut</span>[T]")
                     .finish(&mut r.vector, &mut vector_box);


    // MEMORY
    vector_box.add_section("Memory");

    // capacity
    MethodLine::new().a_add_docs("capacity")
                     .text("  .capacity();")
                     .finish(&mut r.vector, &mut vector_box);

    // reserve
    MethodLine::new().a_add_docs("reserve")
                     .text("  .reserve(100);")
                     .details("in addition to .len() or more")
                     .finish(&mut r.vector, &mut vector_box);

    // reserve_exact
    MethodLine::new().a_add_docs("reserve_exact")
                     .text("  .reserve_exact(100);")
                     .details("in addition to .len()")
                     .finish(&mut r.vector, &mut vector_box);

    // shrink_to_fit
    MethodLine::new().a_add_docs("shrink_to_fit")
                     .text("  .shrink_to_fit();")
                     .finish(&mut r.vector, &mut vector_box);

    // MEMORY
    vector_box.add_section("Split");


    // split_at
    MethodLine::new().a_add_docs("split_at")    .text("  .split_at")
                     .a_add_docs("split_at_mut").text("<span>_mut</span>(mid);")
                     .details("-&gt; (p1, p2), [mid] in 2nd part")
                     .finish(&mut r.vector, &mut vector_box);


    MethodLine::new().a_add_docs("split")       .text("  .split")
                     .a_add_docs("split_mut")   .text("<span>_mut</span>(|&amp;T| -&gt; bool);")
                     .finish(&mut r.vector, &mut vector_box);


    MethodLine::new().a_add_docs("splitn")      .text("  .splitn")
                     .a_add_docs("splitn_mut")  .text("<span>_mut</span>(n, |&amp;T| -&gt; bool); ")
                     .a_add_docs("rsplitn")     .text(".rsplitn")
                     .a_add_docs("rsplitn_mut") .text("<span>_mut</span>(_);")
                     .details("-&gt; iter over <span>mutable</span> subslices,<br> seperated by ||-&gt;true, <span>at most n times</span>")
                     .finish(&mut r.vector, &mut vector_box);


    MethodLine::new().a_add_docs("split_off")
                     .text("  .split_off(mid);")
                     .details("-&gt; Vec; [mid] in 2nd part")
                     .finish(&mut r.vector, &mut vector_box);

    // COMPARISION
    vector_box.add_section("Comparision");

    // cmp
    MethodLine::new().a_add_docs("cmp")     .text("  .cmp() ")
                     .a_add_docs("eq")      .text(".eq() ")
                     .a_add_docs("ne")      .text(".ne();")
                     .details("T: PartialEq")
                     .finish(&mut r.vector, &mut vector_box);

    // lt
    MethodLine::new().a_add_docs("lt") .text("  .lt() ")
                     .a_add_docs("le") .text(".le() ")
                     .a_add_docs("gt") .text(".gt() ")
                     .a_add_docs("ge") .text(".ge();")
                     .details("if T:PartialOrd")
                     .finish(&mut r.vector, &mut vector_box);

    vector_box.add_hr();

    // hash
    MethodLine::new().a_add_docs("hash")
                     .text("  .hash(state: Hasher)")
                     .details("if T:Hash")
                     .finish(&mut r.vector, &mut vector_box);

    // TRAITS
    vector_box.add_section("Traits");

    MethodLine::new()
        .a_add_docs("from")        .text("<code>From&lt;BinaryHeap&gt;</code>,")
        .a_add_docs("borrow")       .text("<code>Borrow")
        .a_add_docs("borrow_mut")   .text("<span>Mut</span></code>,")
        .a_add_docs("clone")        .text("<code>Clone ")
        .a_add_docs("clone_from")   .text("<span>+</span></code>,")
        .a_add_docs("hash")         .text("<code>Hash ")
        .a_add_docs("hash_slice")   .text("<span>+</span></code>,")
        .a_add_docs("index")        .text("<code>Index")
        .a_add_docs("index_mut")    .text("<span>Mut</span></code>,")
        .a_add_docs("deref")        .text("<code>Deref")
        .a_add_docs("deref_mut")    .text("<span>Mut</span></code>,")
        .a("trait.FromIterator")    .text("<code>FromIterator</code>,")
        .a("trait.IntoIterator")    .text("<code>IntoIterator</code>,")
        .a_add_docs("extend")       .text("<code>Extend</code>,")
        .a("trait.PartialEq")       .text("<code>PartialEq</code>,")
        .a("trait.PartialOrd")      .text("<code>PartialOrd</code>,")
        .a("trait.Eq")              .text("<code>Eq</code>,")
        .a("trait.Ord")             .text("<code>Ord</code>,")
        .a_add_docs("drop")         .text("<code>Drop</code>,")
        .a_add_docs("default")      .text("<code>Default</code>,")
        .a_add_docs("fmt")          .text("<code>Debug (if T:Debug)</code>,")
        .a_add_docs("as_ref")       .text("<code>AsRef</code>,")
        .a_add_docs("as_mut")       .text("<code>AsMut</code>,")
        .a_add_docs("from-1")       .text("<code>From</code>,")
        .a("trait.Write")           .text("<code>Write</code>")
        .finish(&mut r.vector, &mut vector_box);

}