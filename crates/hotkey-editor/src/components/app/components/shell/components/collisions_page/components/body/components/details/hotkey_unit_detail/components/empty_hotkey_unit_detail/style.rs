use tw_macro::tw;
// The empty detail pane surface: the base pane layout VALUES plus the centered, muted
// empty-prompt treatment. Shares values with the filled variant; each writes its own list.
classes! {
    base: tw![
        "flex",
        "flex-col",
        "items-center",
        "justify-center",
        "self-stretch",
        "w-full",
        "min-w-0",
        "min-h-64",
        "max-h-full",
        "gap-6",
        "py-4",
        "px-5",
        "border",
        "border-warcraft-blue-deep",
        "rounded-container",
        "bg-panel-dark",
        "overflow-hidden",
        "text-warcraft-text-faint",
        "italic",
    ],
}
