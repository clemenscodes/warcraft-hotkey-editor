use tw_macro::tw;

classes! {
    base: tw![
        "flex-none",
        "flex",
        "items-center",
        "justify-center",
        "w-12",
        "h-12",
        "rounded-full",
        "self-center",
        "[&>svg]:w-8",
        "[&>svg]:h-8",
        "bg-(--toast-accent)/18",
        "text-(--toast-accent)",
    ],
}
