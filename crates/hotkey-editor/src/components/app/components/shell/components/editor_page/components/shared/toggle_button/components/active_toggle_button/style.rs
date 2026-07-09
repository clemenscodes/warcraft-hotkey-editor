use tw_macro::tw;
// The shared gold pill lit as the active choice: the same chrome values as the idle
// sibling plus the always-on gold accent (surface, border, text, glow). Shared values,
// not a shared look — each look-component writes its own class list.

classes! {
    base: tw![
        "flex-1",
        "px-6",
        "border",
        "rounded-panel",
        "text-xl",
        "uppercase",
        "tracking-caps",
        "whitespace-nowrap",
        "text-shadow-drop",
        "cursor-pointer",
        "transition-[border-color,color,box-shadow]",
        "duration-base",
        "bg-panel-gold-active",
        "border-warcraft-gold",
        "text-warcraft-gold",
        "shadow-glow",
        "focus:outline-none",
        "kb-focus:border-white",
        "kb-focus:text-white",
        "kb-focus:shadow-focus",
    ],
    mobile: tw!["mobile:text-base", "mobile:px-2.5"],
    tablet: tw!["tablet:text-md", "tablet:px-4"],
    laptop: tw!["laptop:text-md", "laptop:px-4"],
    desktop: tw!["desktop:text-md", "desktop:px-4"],
}
