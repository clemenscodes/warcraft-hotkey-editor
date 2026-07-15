use tw_macro::tw;

classes! {
    base: tw![
        "inline-flex",
        "items-center",
        "justify-center",
        "size-full",
        "p-0",
        "border-[1.25cqi]",
        "rounded-[15cqi]",
        "cursor-pointer",
        "bg-panel-gold-resting",
        "transition-interactive",
        "focus:outline-none",
        "kb-focus:border-white",
        "kb-focus:text-white",
        "kb-focus:[--focus-color:var(--color-warcraft-highlight)]",
        "kb-focus:shadow-focus",
        "border-warcraft-gold",
        "text-warcraft-gold",
        "shadow-glow-soft",
        "hover:bg-panel-gold-active",
        "hover:shadow-glow",
    ],
    mobile: tw![
        "mobile:border-[2.8cqi]",
    ],
    tablet: tw![
        "tablet:border-[2.8cqi]",
    ],
}
