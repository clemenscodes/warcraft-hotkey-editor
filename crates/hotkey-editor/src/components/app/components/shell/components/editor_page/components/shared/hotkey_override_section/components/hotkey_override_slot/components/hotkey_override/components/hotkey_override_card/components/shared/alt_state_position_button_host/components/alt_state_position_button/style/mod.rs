use tw_macro::tw;

classes! {
    base: tw![
        "appearance-none",
        "size-full",
        "p-1",
        "flex",
        "items-center",
        "justify-center",
        "bg-(--race-color,var(--color-race-human))/8",
        "border-2",
        "border-(--race-color,var(--color-race-human))",
        "text-warcraft-text-secondary",
        "rounded-control",
        "cursor-pointer",
        "transition-[background,border-color,color]",
        "duration-fast",
        "hover:bg-(--race-color,var(--color-race-human))/22",
        "hover:border-(--race-color,var(--color-race-human))",
        "hover:text-warcraft-text-secondary",
        "kb-focus:outline-2",
        "kb-focus:outline-(--race-color,var(--color-race-human))",
        "kb-focus:outline-offset-2",
        "[&>svg]:block",
        "[&>svg]:w-full",
        "[&>svg]:h-full",
    ],
}
