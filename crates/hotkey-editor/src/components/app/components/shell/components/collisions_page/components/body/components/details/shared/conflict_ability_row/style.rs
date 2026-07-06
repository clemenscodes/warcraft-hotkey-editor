use tw_macro::tw;
classes! {
    base: tw![
        "grid",
        "grid-cols-[1fr_auto_1fr]",
        "items-start",
        "justify-items-center",
        "gap-[12px]",
        "w-full",
        "data-[multi=true]:grid-cols-none",
        "data-[multi=true]:flex",
        "data-[multi=true]:flex-wrap",
        "data-[multi=true]:justify-center",
    ],
}
