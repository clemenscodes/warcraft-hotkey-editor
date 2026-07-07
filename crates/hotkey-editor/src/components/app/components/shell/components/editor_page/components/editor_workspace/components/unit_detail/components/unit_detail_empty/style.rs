use tw_macro::tw;
// The unit-detail card in its empty state: the same bordered shell, centered muted
// italic message when no (or an invalid) unit is selected.

classes! {
    base: tw![
        "flex",
        "flex-col",
        "items-center",
        "justify-center",
        "min-w-0",
        "min-h-[16rem]",
        "p-8",
        "border",
        "border-warcraft-blue-deep",
        "rounded-container",
        "bg-panel-dark-diag-70",
        "shadow-bevel-hl-3",
        "text-warcraft-text-faint",
        "text-base",
        "italic",
    ],
    mobile: tw![
        "mobile:p-3.5",
        "mobile:rounded-tile",
    ],
}
