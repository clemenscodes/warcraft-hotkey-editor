use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "grid",
        "grid-cols-[minmax(0,1fr)_auto_auto]",
        "items-baseline",
        "gap-x-[3.08cqi]",
        "gap-y-[calc(0.96cqi+var(--stats-spread,0)*1px)]",
        "min-w-0",
        "text-xl",
        "leading-title",
        "text-shadow-drop",
    ],
    mobile: tw![
        "mobile:text-sm",
        "mobile:leading-heading",
    ],
}
