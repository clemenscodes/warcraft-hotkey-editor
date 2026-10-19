use tw_macro::tw;

classes! {
    // The count rides as a round bubble in the tab's top right corner, the way
    // the carrier count badge sits on an ability icon, so the section name owns
    // the tab on one line and the count never widens or wraps it. Every length
    // is in `em` off the tab's own type, so the circle grows with the tab and
    // stays a circle; it sits inside the tab's border, never on it. The border
    // and text take the tab's own colour through currentColor.
    base: tw![
        "absolute",
        "top-[0.3em]",
        "right-[0.3em]",
        "size-[1.7em]",
        "aspect-square",
        "box-border",
        "inline-flex",
        "items-center",
        "justify-center",
        "rounded-full",
        "border",
        "border-current",
        "bg-warcraft-bg-base",
        "text-[0.8em]",
        "leading-none",
        "tabular-nums",
        "opacity-(--count-opacity,0.9)",
    ],
}
