use tw_macro::tw;
classes! {
    base: tw![
        "group/editable-keycap",
        "[--keycap-radius:var(--radius-tile)]",
        "flex",
        "items-center",
        "justify-center",
        "w-20",
        "h-20",
        "p-0",
        "text-2xl",
        "cursor-pointer",
        "kb-focus:outline-none",
        "kb-focus:shadow-glow-soft",
    ],
    mobile: tw![
        "mobile:w-[4.6rem]",
        "mobile:h-[4.6rem]",
        "mobile:min-w-[4.6rem]",
        "mobile:min-h-[4.6rem]",
        "mobile:text-2xl",
    ],
}
