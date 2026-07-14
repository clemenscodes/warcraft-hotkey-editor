use tw_macro::tw;

classes! {
    base: tw![
        "@container",
        "h-full",
        "w-auto",
        "max-w-full",
        "aspect-square",
        "shrink-0",
        "p-0",
        "flex",
        "items-center",
        "justify-center",
        "bg-warcraft-gold-dark/55",
        "border",
        "border-warcraft-gold-border",
        "rounded-control",
        "cursor-pointer",
        "transition-[border-color,background]",
        "duration-fast",
        "hover:border-warcraft-gold",
        "hover:bg-warcraft-gold/12",
        "kb-focus:outline-none",
        "kb-focus:shadow-focus",
        "[&>span]:block",
        "[&_svg]:size-[71cqi]",
    ],
}
