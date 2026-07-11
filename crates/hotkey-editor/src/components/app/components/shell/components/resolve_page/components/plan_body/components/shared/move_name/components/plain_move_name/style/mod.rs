use tw_macro::tw;
// The non-clickable ability name (no owning unit to link to). Shared name typography VALUES with its sibling; each writes its own list.
classes! {
    base: tw![
        "text-xl",
        "text-warcraft-gold",
        "whitespace-nowrap",
        "min-w-0",
    ],
    mobile: tw![
        "mobile:text-base",
    ],
}
