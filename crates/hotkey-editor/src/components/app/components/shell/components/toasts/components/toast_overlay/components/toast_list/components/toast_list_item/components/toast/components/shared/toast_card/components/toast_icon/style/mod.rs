use tw_macro::tw;

classes! {
    base: tw![
        "flex-none",
        "flex",
        "items-center",
        "justify-center",
        "size-[3em]",
        "rounded-full",
        "self-center",
        "[&>svg]:size-[2em]",
        "bg-(--toast-accent)/18",
        "text-(--toast-accent)",
    ],
}
