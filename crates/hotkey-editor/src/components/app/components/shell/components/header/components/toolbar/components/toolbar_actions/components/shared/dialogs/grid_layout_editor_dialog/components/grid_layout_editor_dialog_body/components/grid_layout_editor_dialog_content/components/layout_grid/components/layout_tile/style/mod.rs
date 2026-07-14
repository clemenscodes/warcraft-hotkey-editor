use tw_macro::tw;

classes! {
    base: tw![
        "@container",
        "group/editable-keycap",
        "flex",
        "items-center",
        "justify-center",
        "size-full",
        "p-0",
        "text-5xl",
        "focus:outline-none",
        "kb-focus:outline-none",
    ],
    mobile: tw![
        "mobile:text-2xl",
    ],
    tablet: tw![
        "tablet:text-2xl",
    ],
}
