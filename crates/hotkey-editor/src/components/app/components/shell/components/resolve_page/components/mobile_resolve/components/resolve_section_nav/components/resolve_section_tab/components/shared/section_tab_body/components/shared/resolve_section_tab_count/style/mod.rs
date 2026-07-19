use tw_macro::tw;

classes! {
    // The count rides as a small bubble in the tab's top right corner, the way
    // the carrier count badge sits on an ability icon, so the section name owns
    // the tab on one line and the count never widens or wraps it. The border
    // and text take the tab's own colour through currentColor.
    base: tw![
        "absolute",
        "top-0",
        "right-0.5",
        "min-w-[0.9rem]",
        "h-[0.9rem]",
        "px-1",
        "box-border",
        "inline-flex",
        "items-center",
        "justify-center",
        "rounded-full",
        "border",
        "border-current",
        "bg-warcraft-bg-base",
        "text-[0.55rem]",
        "leading-none",
        "opacity-(--count-opacity,0.9)",
    ],
}
