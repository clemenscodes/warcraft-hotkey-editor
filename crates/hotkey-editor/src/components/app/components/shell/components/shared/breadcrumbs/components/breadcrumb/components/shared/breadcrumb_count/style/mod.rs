use tw_macro::tw;
classes! {
    base: tw![
        "text-xl",
        "opacity-(--count-opacity,0.8)",
        "before:content-['(']",
        "after:content-[')']",
    ],
}
