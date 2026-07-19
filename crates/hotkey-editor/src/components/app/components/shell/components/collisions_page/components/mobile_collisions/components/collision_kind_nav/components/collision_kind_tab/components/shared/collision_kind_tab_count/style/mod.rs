use tw_macro::tw;

classes! {
    base: tw![
        "opacity-(--count-opacity,0.8)",
        "before:content-['(']",
        "after:content-[')']",
    ],
}
