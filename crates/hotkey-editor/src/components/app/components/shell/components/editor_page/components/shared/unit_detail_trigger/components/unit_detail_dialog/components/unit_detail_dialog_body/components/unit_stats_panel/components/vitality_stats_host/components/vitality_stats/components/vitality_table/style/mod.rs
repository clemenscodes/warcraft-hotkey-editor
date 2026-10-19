use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "grid",
        "grid-cols-[minmax(0,1fr)_auto_auto]",
        "items-baseline",
        "gap-x-[1.25em]",
        "gap-y-[0.3em]",
        "min-w-0",
        "leading-heading",
        "text-shadow-drop",
    ],
    mobile: tw![
        "mobile:gap-x-[3.08cqi]",
        "mobile:gap-y-[0.96cqi]",
    ],
}
