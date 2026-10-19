use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "grid",
        "col-span-2",
        "self-end",
        "grid-cols-2",
        "gap-x-[0.5em]",
        "gap-y-[0.3em]",
        "pt-[0.75em]",
        "border-t",
        "border-t-warcraft-gold/15",
    ],
    mobile: tw![
        "mobile:gap-x-[1.54cqi]",
        "mobile:gap-y-[0.77cqi]",
        "mobile:pt-[1.54cqi]",
    ],
}
