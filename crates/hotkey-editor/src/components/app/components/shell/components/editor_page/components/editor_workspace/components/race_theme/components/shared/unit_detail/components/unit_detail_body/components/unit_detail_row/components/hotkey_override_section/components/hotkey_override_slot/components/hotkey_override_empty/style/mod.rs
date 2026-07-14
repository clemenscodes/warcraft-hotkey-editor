use tw_macro::tw;
// The placeholder shown in the hotkey-override section before a grid tile is
// selected: a dashed muted-italic box.

classes! {
    base: tw![
        "flex",
        "flex-col",
        "items-center",
        "justify-center",
        "flex-none",
        "overflow-hidden",
        "py-8",
        "px-9",
        "border",
        "border-dashed",
        "border-warcraft-blue",
        "rounded-panel",
        "bg-warcraft-bg-mid/45",
        "text-warcraft-text-faint",
        "text-xl",
        "leading-body",
        "italic",
    ],
    mobile: tw![
        "mobile:w-full",
        "mobile:box-border",
        "mobile:py-2.5",
        "mobile:px-3",
        "mobile:text-center",
        "mobile:text-sm",
        "mobile:leading-body",
    ],
    tablet: tw![
        "tablet:w-full",
        "tablet:box-border",
        "tablet:py-2.5",
        "tablet:px-3",
        "tablet:text-sm",
        "tablet:leading-body",
    ],
}
