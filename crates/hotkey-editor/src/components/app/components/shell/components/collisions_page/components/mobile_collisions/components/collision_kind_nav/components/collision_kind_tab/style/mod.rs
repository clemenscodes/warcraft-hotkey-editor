use tw_macro::tw;

// One button whichever kind is active, so its look moves between the idle and
// the current state on a transition instead of the button being rebuilt.
classes! {
    base: tw![
        "flex",
        "flex-1",
        "min-w-0",
        "flex-row",
        "items-center",
        "justify-center",
        "gap-[0.4em]",
        "px-1",
        "border",
        "border-warcraft-blue-deep",
        "rounded-card",
        "bg-panel-dark",
        "uppercase",
        "text-warcraft-text-muted",
        "transition-[color,border-color,box-shadow,background-color]",
        "duration-base",
        "aria-current:border-warcraft-gold/70",
        "aria-current:bg-warcraft-gold/10",
        "aria-current:text-warcraft-gold",
        "aria-current:text-shadow-drop",
        "aria-[current=false]:hover:text-warcraft-text-primary",
        "aria-[current=false]:hover:border-warcraft-gold/70",
        "hover:shadow-glow",
        "kb-focus:outline-none",
        "kb-focus:shadow-focus",
    ],
}
