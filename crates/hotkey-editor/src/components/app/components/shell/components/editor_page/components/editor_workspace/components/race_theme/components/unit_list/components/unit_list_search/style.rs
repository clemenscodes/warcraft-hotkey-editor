use tw_macro::tw;
// The search box wrapper. On the sidebar it is a bordered inset panel; on mobile it
// becomes a bare relative box (the input carries its own chrome there) so the icon
// can be positioned over the field.

classes! {
    base: tw![
        "flex-none",
        "flex",
        "items-center",
        "gap-2",
        "p-2",
        "bg-warcraft-bg-mid/85",
        "border",
        "border-warcraft-blue-deep",
        "rounded-tile",
        "min-w-0",
    ],
    mobile: tw![
        "mobile:relative",
        "mobile:p-0",
        "mobile:bg-transparent",
        "mobile:border-0",
        "mobile:rounded-none",
    ],
    tablet: tw![
        "tablet:relative",
        "tablet:p-0",
        "tablet:bg-transparent",
        "tablet:border-0",
        "tablet:rounded-none",
    ],
}
