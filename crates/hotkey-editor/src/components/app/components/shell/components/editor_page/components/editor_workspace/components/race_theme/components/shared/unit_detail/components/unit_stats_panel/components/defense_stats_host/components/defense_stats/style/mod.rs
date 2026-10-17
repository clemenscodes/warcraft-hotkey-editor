use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "grid",
        "grid-cols-[auto_minmax(0,1fr)]",
        "items-stretch",
        "gap-x-[3.74cqi]",
        "gap-y-[calc(1.2cqi+var(--stats-spread,0)*1px)]",
        "min-w-0",
        "relative",
        "size-full",
    ],
}
