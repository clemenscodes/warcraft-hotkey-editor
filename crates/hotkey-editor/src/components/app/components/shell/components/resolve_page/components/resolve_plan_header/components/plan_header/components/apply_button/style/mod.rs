use tw_macro::tw;
classes! {
    base: tw![
        "flex-none",
        "px-6",
        "py-2",
        "border",
        "border-warcraft-gold",
        "rounded-card",
        "cursor-pointer",
        "text-2xl",
        "text-warcraft-gold",
        "bg-panel-blue",
        "text-shadow-drop",
        "transition-[box-shadow,background]",
        "duration-fast",
        "hover:bg-panel-blue",
        "hover:shadow-glow",
        "kb-focus:outline-none",
        "kb-focus:shadow-focus",
        "disabled:opacity-60",
        "disabled:cursor-wait",
    ],
    // On the phone apply bar the button steps down so it does not dominate the
    // compact header next to the plan summary.
    mobile: tw![
        "mobile:px-3",
        "mobile:py-1",
        "mobile:text-sm",
    ],
}
