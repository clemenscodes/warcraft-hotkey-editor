use tw_macro::tw;
// The placeholder shown in the override panel before a grid tile is selected: a
// dashed muted-italic box.

classes! {
    base: tw![
        "flex",
        "flex-col",
        "items-center",
        "justify-center",
        "flex-[0_0_auto]",
        "overflow-hidden",
        "py-8", "px-9",
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
        "mobile:h-75",
        "mobile:py-2.5", "mobile:px-3",
        "mobile:text-center",
        "mobile:text-sm",
        "mobile:leading-body",
        "mobile:rounded-t-container",
        "mobile:border-b-0",
        "mobile:shadow-drop-top",
    ],
    tablet: tw![
        "tablet:w-full",
        "tablet:box-border",
        "tablet:h-75",
        "tablet:py-2.5", "tablet:px-3",
        "tablet:text-sm",
        "tablet:leading-body",
    ],
}
