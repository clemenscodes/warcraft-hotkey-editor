use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "flex",
        "flex-col",
        "py-[2.55cqi]",
        "bg-warcraft-bg-mid/45",
        "border",
        "rounded-panel",
        "gap-[2.13cqi]",
        "px-[2.55cqi]",
        "box-border",
        "border-warcraft-blue-deep",
    ],
    // The panel fills the full height pager card on a phone so the bordered
    // card reads like the unit card rather than a small block of content.
    mobile: tw![
        "mobile:h-full",
        "mobile:min-h-0",
        "mobile:overflow-hidden",
    ],
}
