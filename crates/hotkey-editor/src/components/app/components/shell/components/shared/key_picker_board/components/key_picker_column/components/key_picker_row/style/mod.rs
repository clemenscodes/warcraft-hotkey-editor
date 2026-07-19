use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "justify-center",
        "gap-1",
    ],
    // A keyboard row is wider than a phone. Rather than shrink the keys until
    // their labels no longer fit, let the row break onto as many lines as it
    // needs and keep the keys at a legible size.
    mobile: tw![
        "mobile:gap-0.5",
        "mobile:flex-wrap",
        "mobile:[justify-content:var(--key-row-justify,center)]",
    ],
}
