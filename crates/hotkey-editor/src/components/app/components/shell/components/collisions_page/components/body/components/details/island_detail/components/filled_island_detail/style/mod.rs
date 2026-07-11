use tw_macro::tw;
// The populated detail pane surface. Shares the base pane layout VALUES with the empty
// variant; each variant writes its own list.
classes! {
    base: tw![
        "flex",
        "flex-col",
        "self-stretch",
        "w-full",
        "min-w-0",
        "min-h-0",
        "max-h-full",
        "gap-6",
        "py-4",
        "px-5",
        "border",
        "border-warcraft-blue-deep",
        "rounded-container",
        "bg-panel-dark",
        "overflow-hidden",
    ],
}
