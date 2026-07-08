use tw_macro::tw;
// The object id under the name, in a monospace face; smaller on the mobile panel.

classes! {
    base: tw![
        "text-lg",
        "text-warcraft-text-faint",
    ],
    mobile: tw![
        "mobile:m-0",
        "mobile:text-xs",
        "mobile:leading-title",
        "mobile:overflow-hidden",
        "mobile:whitespace-nowrap",
        "mobile:text-ellipsis",
    ],
    tablet: tw![
        "tablet:m-0",
        "tablet:text-xs",
        "tablet:leading-title",
        "tablet:overflow-hidden",
        "tablet:whitespace-nowrap",
        "tablet:text-ellipsis",
    ],
}
