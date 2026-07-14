use tw_macro::tw;
classes! {
    base: tw![
        "group/editable-keycap",
        "[--keycap-radius:var(--radius-tile)]",
        "flex",
        "items-center",
        "justify-center",
        "w-auto",
        "min-w-20",
        "h-20",
        "p-0",
        "text-xl",
        "whitespace-nowrap",
        "tracking-normal",
        "cursor-pointer",
        "kb-focus:outline-none",
        "kb-focus:shadow-glow-soft",
    ],
    mobile: tw![
        "mobile:w-auto",
        "mobile:h-[4.6rem]",
        "mobile:min-w-[4.6rem]",
        "mobile:min-h-[4.6rem]",
        "mobile:text-xl",
    ],
}
