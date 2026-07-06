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
        "p-[2rem_2.25rem]",
        "border",
        "border-dashed",
        "border-warcraft-blue-bright",
        "rounded-panel",
        "bg-warcraft-bg-mid/45",
        "text-warcraft-text-faint",
        "text-[1.8rem]",
        "leading-[1.45]",
        "italic",
    ],
    mobile: tw![
        "mobile:w-full",
        "mobile:box-border",
        "mobile:h-[300px]",
        "mobile:p-[10px_12px]",
        "mobile:text-center",
        "mobile:text-[14px]",
        "mobile:leading-[1.4]",
        "mobile:rounded-[12px_12px_0_0]",
        "mobile:border-b-0",
        "mobile:shadow-[0_-4px_16px_color-mix(in_oklab,var(--color-warcraft-shadow)_40%,transparent)]",
    ],
    tablet: tw![
        "tablet:w-full",
        "tablet:box-border",
        "tablet:h-[300px]",
        "tablet:p-[10px_12px]",
        "tablet:text-[14px]",
        "tablet:leading-[1.4]",
    ],
}
