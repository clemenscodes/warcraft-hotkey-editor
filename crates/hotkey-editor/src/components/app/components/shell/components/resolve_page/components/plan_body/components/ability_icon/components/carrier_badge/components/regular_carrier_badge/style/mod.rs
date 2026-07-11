use tw_macro::tw;
// The carrier-count badge when its ability does not win the cell. Shared badge chrome VALUES with its sibling; each writes its own list.
classes! {
    base: tw![
        "absolute",
        "-top-1.5",
        "-right-1.5",
        "min-w-4.75",
        "h-4.75",
        "px-1",
        "box-border",
        "inline-flex",
        "items-center",
        "justify-center",
        "rounded-panel",
        "bg-warcraft-bg-panel",
        "border",
        "text-base",
        "leading-none",
        "text-shadow-drop",
        "border-warcraft-blue",
        "text-warcraft-text-secondary",
    ],
    mobile: tw![
        "mobile:min-w-3.75",
        "mobile:h-3.75",
        "mobile:text-sm",
    ],
}
