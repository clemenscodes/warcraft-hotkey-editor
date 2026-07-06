use tw_macro::tw;
classes! {
    base: tw![
        "w-[72px]",
        "h-[72px]",
        "border",
        "border-warcraft-blue",
        "rounded-[7px]",
        "object-cover",
        "group-[:not(:disabled):hover]:border-warcraft-gold",
        "group-[:not(:disabled):hover]:shadow-glow-8",
    ],
    mobile: tw![
        "mobile:w-[max(40px,min(72px,9vw))]",
        "mobile:h-[max(40px,min(72px,9vw))]",
    ],
}
