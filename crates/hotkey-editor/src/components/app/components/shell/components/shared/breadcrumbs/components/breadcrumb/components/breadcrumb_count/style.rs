use tw_macro::tw;
classes! {
    base: tw![
        "text-xl",
        "opacity-80",
        "before:content-['(']",
        "after:content-[')']",
        "group-data-[active=true]:opacity-100",
    ],
}
