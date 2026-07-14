use tw_macro::tw;
// The crosshair button that opens the position picker for an off-state / upgraded-
// form button. Blue-themed square matching the alt-state block; the crosshair SVG
// fills it. Shared by the alt-state and upgrade sections.

classes! {
    base: tw![
        "appearance-none",
        "w-20",
        "h-20",
        "p-1",
        "inline-flex",
        "items-center",
        "justify-center",
        "bg-race-human/8",
        "border-2",
        "border-race-human",
        "text-warcraft-text-secondary",
        "rounded-control",
        "cursor-pointer",
        "transition-[background,border-color,color]",
        "duration-fast",
        "hover:bg-race-human/22",
        "hover:border-race-human",
        "hover:text-warcraft-text-secondary",
        "kb-focus:outline-2",
        "kb-focus:outline-race-human",
        "kb-focus:outline-offset-2",
        "[&>svg]:block",
        "[&>svg]:w-full",
        "[&>svg]:h-full",
    ],
    mobile: tw![
        "mobile:w-[4.6rem]",
        "mobile:h-[4.6rem]",
    ],
}
