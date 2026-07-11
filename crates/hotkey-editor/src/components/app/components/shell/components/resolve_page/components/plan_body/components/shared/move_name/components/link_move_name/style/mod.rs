use tw_macro::tw;
// The clickable ability name that deep-links into the editor; underlines on the button's hover. Shared name typography VALUES with its sibling; each writes its own list.
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
