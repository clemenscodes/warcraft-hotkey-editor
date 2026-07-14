use tw_macro::tw;
classes! {
    base: tw![
        "text-xl",
        "text-warcraft-gold",
        "whitespace-nowrap",
        "min-w-0",
        "cursor-pointer",
        "group-[:not(:disabled):hover]:text-white",
        "group-[:not(:disabled):hover]:underline",
        "group-[:not(:disabled):hover]:underline-offset-2",
    ],
    mobile: tw![
        "mobile:text-base",
    ],
}
