use tw_macro::tw;
classes! {
    base: tw![
        "text-md",
        "leading-none",
        "shrink-0",
        "transition-[transform]", "duration-slow",
        "group-data-[open=true]:rotate-180",
    ],
}
