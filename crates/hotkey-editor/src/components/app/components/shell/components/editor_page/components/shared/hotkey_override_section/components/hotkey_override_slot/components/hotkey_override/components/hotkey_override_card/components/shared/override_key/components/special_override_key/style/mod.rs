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
        "h-full",
        "p-0",
        "text-xl",
        "whitespace-nowrap",
        "tracking-normal",
        "cursor-pointer",
        "kb-focus:outline-none",
        "kb-focus:shadow-glow-soft",
    ],
    mobile: tw![
        "mobile:min-w-[3.5em]",
        "mobile:text-[1.2em]",
    ],
}
