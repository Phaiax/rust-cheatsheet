
use ::{References,Group,sel,MethodLine,Doc};

pub fn make(r : &mut References, mut section : &mut Group) {

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
                     .finish(&mut r.vector, &mut section);

    // with_capacity
    MethodLine::new().a_add_docs("with_capacity")
                .text("            = Vec::with_capacity();")
                .finish(&mut r.vector, &mut section);

    // vec![]
    MethodLine::new().a("vec.initmacro")
                     .text("            = vec![];")
                     .finish(&mut r.vector, &mut section);
    r.vector_macro.add_doc_by_element("vec.initmacro", sel("section#main"));

    // boxed array -> Vec
    MethodLine::new().a_add_docs("into_vec")
                     .text("            = boxedarray.into_vec()")
                     .rarelyused()
                     .finish(&mut r.vector, &mut section);

    // From
    MethodLine::new().a_add_docs("from-1") .text("            = Vec::from(slice|")
                     .a_add_docs("from-2") .text("str|")
                     .a_add_docs("from-3") .text("VecDeque|")
                     .a_add_docs("from-4") .text("CString)")
                     .finish(&mut r.vector, &mut section);

    // clone
    MethodLine::new().a_add_docs("clone")
                     .text("            = othervec.clone();")
                     .details("if T:Clone")
                     .finish(&mut r.vector, &mut section);


    // ACCESSING
    section.add_section("Accessing");

    // v[]
    MethodLine::new().a("vec.elementaccess")
                     .text("vec[3];")
                     .details("vec[1..3], vec[..3], vec[3..], vec[..]; vec[2] = a;")
                     .finish(&mut r.vector, &mut section);
    r.vector.add_doc_by_element_range("vec.elementaccess", sel("#indexing"), sel("#slicing"));


    // len
    MethodLine::new().a_add_docs("len")
                     .text("vec.len();")
                     .finish(&mut r.vector, &mut section);

    // is_empty
    MethodLine::new().single_method_with_doc("is_empty")
                     .finish(&mut r.vector, &mut section);

    // first_mut, last_mut
    MethodLine::new().a_add_docs("first")       .text("  .first")
                     .a_add_docs("first_mut")   .text("<span>_mut</span>(); ")
                     .a_add_docs("last")        .text(".last")
                     .a_add_docs("last_mut")    .text("<span>_mut</span>();")
                     .details("-&gt; Option")
                     .finish(&mut r.vector, &mut section);

    // get_mut
    MethodLine::new().a_add_docs("get")     .text("  .get")
                                 .a_add_docs("get_mut")  .text("<span>_mut</span>(index);")
                    .details("-&gt; Option")
                    .finish(&mut r.vector, &mut section);


    // contains
    MethodLine::new().a_add_docs("contains")
                     .text("  .contains(needle);")
                     .details("-&gt; bool")
                     .finish(&mut r.vector, &mut section);

    // find
    MethodLine::new().a("iter.find")
                     .text("  .iter().find(|&amp;T| -> bool);")
                     .details("-&gt; Option")
                     .tags("search scan")
                     .finish(&mut r.vector, &mut section);
    r.iter.add_doc_for_method("iter.find", "find");

    // binary_search
    MethodLine::new().a_add_docs("binary_search")
                     .text("  .binary_search(x:&amp;T);")
                     .details("-&gt; Result&lt;usize, usize&gt;<br>Ok(i): pos, Err(i): pos for insertion")
                     .finish(&mut r.vector, &mut section);

    // binary_search_by
    MethodLine::new().a_add_docs("binary_search_by")
                     .text("  .binary_search_by(|&amp;T|->Ordering);")
                     .rarelyused()
                     .finish(&mut r.vector, &mut section);

    // binary_search_by_key
    MethodLine::new().a_add_docs("binary_search_by_key")
                     .text("  .binary_search_by_key(Key, |&amp;T|->Key);")
                     .rarelyused()
                     .finish(&mut r.vector, &mut section);

    // ends_with
    MethodLine::new().a_add_docs("ends_with")
                     .text("  .ends_with(needle);")
                     .rarelyused()
                     .finish(&mut r.vector, &mut section);

    // starts_with
    MethodLine::new().a_add_docs("starts_with")
                     .text("  .starts_with(needle);")
                     .rarelyused()
                     .finish(&mut r.vector, &mut section);

    // ADDING
    section.add_section("Adding");

    // push
    MethodLine::new().a_add_docs("push")
                     .text("  .push(3);")
                     .details("to end")
                     .finish(&mut r.vector, &mut section);

    // insert
    MethodLine::new().a_add_docs("insert")
                     .text("  .insert(index, element);")
                     .finish(&mut r.vector, &mut section);

    // extend
    MethodLine::new().a_add_docs("extend")
                     .text("  .extend(iterable);")
                     .finish(&mut r.vector, &mut section);

    // extend_from_slice
    MethodLine::new().a_add_docs("extend_from_slice")
                     .text("  .extend_from_slice(&amp;[T]);")
                     .finish(&mut r.vector, &mut section);

    // append
    MethodLine::new().a_add_docs("append")
                     .text("  .append(other : Vec);")
                     .details("drains other")
                     .finish(&mut r.vector, &mut section);

    // clone_from
    MethodLine::new().a_add_docs("clone_from")
                     .text("  .clone_from(&amp;Vec&lt;T&gt;);")
                     .details("overrides self, if T:Clone")
                     .rarelyused()
                     .finish(&mut r.vector, &mut section);

    // clone_from_slice
    MethodLine::new().a_add_docs("clone_from_slice")
                     .text("  .clone_from_slice(&amp;[T]);")
                     .details("if T:Clone")
                     .rarelyused()
                     .finish(&mut r.vector, &mut section);

    // copy_from_slice
    MethodLine::new().a_add_docs("copy_from_slice")
                     .text("  .copy_from_slice(&amp;[T]);")
                     .details("if T:Copy (use memcpy)")
                     .rarelyused()
                     .finish(&mut r.vector, &mut section);

    // REMOVING
    section.add_section("Removing");

    // pop
    MethodLine::new().a_add_docs("pop")
                     .text("  .pop();")
                     .details("removes last -> Option")
                     .finish(&mut r.vector, &mut section);

    // remove
    MethodLine::new().a_add_docs("remove")
                     .text("  .remove(index);")
                     .details("-&gt; el, shifts left")
                     .finish(&mut r.vector, &mut section);

    // swap_remove
    MethodLine::new().a_add_docs("swap_remove")
                     .text("  .swap_remove(index);")
                     .details("-&gt; el, fills with last")
                     .finish(&mut r.vector, &mut section);

    // truncate
    MethodLine::new().a_add_docs("truncate")
                     .text("  .truncate(i);")
                     .details("cut until .len() = i")
                     .rarelyused()
                     .finish(&mut r.vector, &mut section);

    // drain
    MethodLine::new().a_add_docs("drain")
                     .text("  .drain(range);")
                     .details("-&gt; iter that drains")
                     .finish(&mut r.vector, &mut section);

    // clear
    MethodLine::new().a_add_docs("clear")
                     .text("  .clear();")
                     .finish(&mut r.vector, &mut section);

    // retain
    MethodLine::new().a_add_docs("retain")
                     .text("  .retain(|i| -&gt; bool);")
                     .details("in place")
                     .finish(&mut r.vector, &mut section);

    // dedup
    MethodLine::new().a_add_docs("dedup")
                     .text("  .dedup();")
                     .rarelyused()
                     .details("removes duplicates (if T:PartialEq)")
                     .finish(&mut r.vector, &mut section);

    // MANIPULATING
    section.add_section("Manipulating");

    // sort
    MethodLine::new().a_add_docs("sort")
                     .text("  .sort();")
                     .details("in place")
                     .finish(&mut r.vector, &mut section);

    // sort_by
    MethodLine::new().a_add_docs("sort_by")
                     .text("  .sort_by(|&amp;T|->Ordering);")
                     .details("in place")
                     .finish(&mut r.vector, &mut section);

    // sort_by_key
    MethodLine::new().a_add_docs("sort_by_key")
                     .text("  .sort_by_key(|&amp;T|->Key);")
                     .details("Key:Ordering")
                     .finish(&mut r.vector, &mut section);

    // reverse
    MethodLine::new().a_add_docs("reverse")
                     .text("  .reverse();")
                     .details("in place")
                     .finish(&mut r.vector, &mut section);

    // swap
    MethodLine::new().a_add_docs("swap")
                     .text("  .swap(index1, index2);")
                     .finish(&mut r.vector, &mut section);


    // TRANSFORMING
    section.add_section("Transforming (Iter, as_, to_)");

    // iter
    MethodLine::new().a_add_docs("iter")     .text("  .iter")
                     .a_add_docs("iter_mut") .text("<span>_mut</span>();")
                     .details("<div title=\"borrows value\">-&gt;&<span>mut </span>T, keeps vector</div>")
                     .finish(&mut r.vector, &mut section);

    // into_iter
    MethodLine::new().a_add_docs("into_iter")
                     .text("  .into_iter();")
                     .details("<div title=\"transfers ownership\">-&gt;T, consumes vector</div>")
                     .finish(&mut r.vector, &mut section);

    // chunks
    MethodLine::new().a_add_docs("chunks")      .text("  .chunks")
                     .a_add_docs("chunks_mut")  .text("<span>_mut</span>(cnk_sz);")
                     .details("-&gt; iter over a non overlapping slice at a time")
                     .finish(&mut r.vector, &mut section);

    // windows
    MethodLine::new().a_add_docs("windows")
                     .text("  .windows(wnd_sz);")
                     .details("-&gt; iter over an overlapping slice at a time")
                     .finish(&mut r.vector, &mut section);
    section.add_hr();

    // into_boxed_slice
    MethodLine::new().a_add_docs("into_boxed_slice")
                     .text("  .into_boxed_slice();")
                     .details("-&gt; Box&lt;T&gt;")
                     .rarelyused()
                     .finish(&mut r.vector, &mut section);

    // as_ref
    MethodLine::new().a_add_docs("as_ref")
                     .text("  .as_ref();")
                     .details("-&gt; &amp;[T] or &amp;Vec&lt;T&gt;")
                     .finish(&mut r.vector, &mut section);

    // to_vec
    MethodLine::new().a_add_docs("to_vec")
                     .text("  .to_vec();")
                     .details("like clone(), if T:Clone")
                     .rarelyused()
                     .finish(&mut r.vector, &mut section);

    // as_slice
    MethodLine::new().a_add_docs("as_slice")     .text("  .as")
                     .a_add_docs("as_mut_slice") .text("<span>_mut</span>")
                                                 .text("_slice();")
                     .details("-&gt; &amp;<span>mut</span>[T]")
                     .finish(&mut r.vector, &mut section);


    // MEMORY
    section.add_section("Memory");

    // capacity
    MethodLine::new().a_add_docs("capacity")
                     .text("  .capacity();")
                     .rarelyused()
                     .finish(&mut r.vector, &mut section);

    // reserve
    MethodLine::new().a_add_docs("reserve")
                     .text("  .reserve(100);")
                     .details("in addition to .len() or more")
                     .finish(&mut r.vector, &mut section);

    // reserve_exact
    MethodLine::new().a_add_docs("reserve_exact")
                     .text("  .reserve_exact(100);")
                     .details("in addition to .len()")
                     .finish(&mut r.vector, &mut section);

    // shrink_to_fit
    MethodLine::new().a_add_docs("shrink_to_fit")
                     .text("  .shrink_to_fit();")
                     .rarelyused()
                     .finish(&mut r.vector, &mut section);

    // MEMORY
    section.add_section("Split");


    // split_at
    MethodLine::new().a_add_docs("split_at")    .text("  .split_at")
                     .a_add_docs("split_at_mut").text("<span>_mut</span>(mid);")
                     .details("-&gt; (p1, p2), [mid] in 2nd part")
                     .finish(&mut r.vector, &mut section);


    MethodLine::new().a_add_docs("split")       .text("  .split")
                     .a_add_docs("split_mut")   .text("<span>_mut</span>(|&amp;T| -&gt; bool);")
                     .finish(&mut r.vector, &mut section);


    MethodLine::new().a_add_docs("splitn")      .text("  .splitn")
                     .a_add_docs("splitn_mut")  .text("<span>_mut</span>(n, |&amp;T| -&gt; bool); ")
                     .a_add_docs("rsplitn")     .text(".rsplitn")
                     .a_add_docs("rsplitn_mut") .text("<span>_mut</span>(_);")
                     .details("-&gt; iter over <span>mutable</span> subslices,<br> seperated by ||-&gt;true, <span>at most n times</span>")
                     .finish(&mut r.vector, &mut section);


    MethodLine::new().a_add_docs("split_off")
                     .text("  .split_off(mid);")
                     .details("-&gt; Vec; [mid] in 2nd part")
                     .finish(&mut r.vector, &mut section);

    // COMPARISION
    section.add_section("Comparision");

    // cmp
    MethodLine::new().a_add_docs("cmp")     .text("  .cmp() ")
                     .a_add_docs("eq")      .text(".eq() ")
                     .a_add_docs("ne")      .text(".ne();")
                     .rarelyused()
                     .details("T: PartialEq")
                     .finish(&mut r.vector, &mut section);

    // lt
    MethodLine::new().a_add_docs("lt") .text("  .lt() ")
                     .a_add_docs("le") .text(".le() ")
                     .a_add_docs("gt") .text(".gt() ")
                     .a_add_docs("ge") .text(".ge();")
                     .rarelyused()
                     .details("if T:PartialOrd")
                     .finish(&mut r.vector, &mut section);

    section.add_hr();

    // hash
    MethodLine::new().a_add_docs("hash")
                     .text("  .hash(state: Hasher)")
                     .rarelyused()
                     .details("if T:Hash")
                     .finish(&mut r.vector, &mut section);

    // TRAITS
    section.add_section("Traits");

    MethodLine::new()
        .a("trait.From")                       .text("From&lt;BinaryHeap&gt;") .br()
        .a_select_add_docs(Doc::Impl("from".into())).span("from() |") .br()

        .a("trait.Borrow")                       .text("Borrow")
        .a("trait.BorrowMut")                       .span("Mut") .br()
        .a_select_add_docs(Doc::Impl("borrow".into())).span("borrow") .br()
        .a_select_add_docs(Doc::Impl("borrow_mut".into())).span("/_mut() |") .br()

        .a("trait.Clone")                       .text("Clone") .br()
        .a_select_add_docs(Doc::Impl("clone".into())).span("clone/_from() |") .br()

        .a("trait.Hash")                       .text("Hash") .br()
        .a_select_add_docs(Doc::Impl("hash".into())).span("hash/_slice() |") .br()

        .a("trait.Index")                .text("Index")
        .a("trait.IndexMut")             .span("Mut") .br()
        .a_select_add_docs(Doc::Nav("index".into(), "pr".into(), "pnnnnnnnnnnnnnnnnnnnnnnnnnnn".into())).span("index/_mut()") .br()

        .a("trait.Deref")                .text("Deref")
        .a("trait.DerefMut")             .span("Mut") .br()
        .a_select_add_docs(Doc::Impl("deref".into())).span("deref")
        .a_select_add_docs(Doc::Impl("deref_mut".into())).span("/_mut() |") .br()

        .a("trait.FromIterator")                .text("FromIterator") .br()
        .a_select_add_docs(Doc::Impl("from_iter".into())).span("from_iter() |") .br()
        .a("trait.IntoIterator")                .text("IntoIterator") .br()
        .a_select_add_docs(Doc::Nav("into_iter".into(), "pr".into(), "pnnnnn".into())).span("into_iter() |") .br()

        .a("trait.Extend")                .text("Extend") .br()
        .a_select_add_docs(Doc::LastImpl("extend".into())).span("extend() |") .br()

        .a("trait.PartialEq")                   .text("PartialEq") .br()
        .a_select_add_docs(Doc::Impl("eq".into())).span("eq() ne() |") .br()

        .a("trait.PartialOrd")                  .text("PartialOrd") .br()
        .a_select_add_docs(Doc::Impl("partial_cmp".into())).span("partial_cmp() lt() le() gt() ge() |") .br()

        .a("trait.Eq")                          .text("Eq |") .br()

        .a("trait.Ord")                         .text("Ord") .br()
        .a_select_add_docs(Doc::Impl("cmp".into()))   .span("cmp() |") .br()

        .a("trait.Drop")                         .text("Drop") .br()
        .a_select_add_docs(Doc::Impl("drop".into()))   .span("drop() |") .br()

        .a("trait.Default")                       .text("Default") .br()
        .a_select_add_docs(Doc::Impl("default".into()))   .span("default() |") .br()

        .a("trait.Debug")                       .text("Debug (if T:Debug)") .br()
        .a_select_add_docs(Doc::Impl("fmt".into()))   .span("fmt() |") .br()

        .a("trait.AsRef")                       .text("AsRef") .br()
        .a("trait.AsMut")                       .text("AsMut") .br()
        .a_select_add_docs(Doc::Nav("as_ref".into(), "pr".into(), "pnnnnnnn".into())).span("as_ref() as_mut() |") .br()

        .a("trait.From")                       .text("From") .br()
        .a_select_add_docs(Doc::Nav("from-1".into(), "pr".into(), "pnnnnnnn".into())).span("from() |") .br()

        .a("trait.Write")                       .text("Write") .br()
        .a_select_add_docs(Doc::LastImpl("write".into()))   .span("write() write_all() flush() by_ref() .. |") .br()
        .finish(&mut r.vector, &mut section);

}