use tw_macro::tw;
// The ability / unit name heading in the hotkey-override section. Gold display face, ellipsized
// on one line; smaller on the mobile panel.

classes! {
    base: tw![
        "max-w-full",
        "overflow-hidden",
        "whitespace-nowrap",
        "text-ellipsis",
        "font-normal",
        "text-2xl",
        "leading-title",
        "text-warcraft-gold",
        "text-shadow-drop",
    ],
    mobile: tw![
        "mobile:text-sm",
        "mobile:break-normal",
    ],
    tablet: tw![
        "tablet:text-sm",
        "tablet:break-normal",
    ],
}
