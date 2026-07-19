use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "grid",
    ],
    // In the mobile pager one move fills a full viewport height card, so it
    // grows to fill its snap slot rather than sitting centred with large gaps.
    mobile: tw![
        "mobile:flex-1",
        "mobile:min-h-0",
    ],
}
