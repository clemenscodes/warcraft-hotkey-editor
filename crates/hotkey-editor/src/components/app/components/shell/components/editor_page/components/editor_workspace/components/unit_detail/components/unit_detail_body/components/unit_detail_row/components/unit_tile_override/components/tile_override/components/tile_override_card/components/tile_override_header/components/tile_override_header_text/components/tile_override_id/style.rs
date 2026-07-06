use tw_macro::tw;
// The object id under the name, in a monospace face; smaller on the mobile panel.

classes! {
    base: tw![
        "text-[1.4rem]",
        "text-warcraft-text-faint",
    ],
    mobile: tw![
        "mobile:m-0",
        "mobile:text-[12px]",
        "mobile:leading-[1.2]",
        "mobile:overflow-hidden",
        "mobile:whitespace-nowrap",
        "mobile:text-ellipsis",
    ],
    tablet: tw![
        "tablet:m-0",
        "tablet:text-[12px]",
        "tablet:leading-[1.2]",
        "tablet:overflow-hidden",
        "tablet:whitespace-nowrap",
        "tablet:text-ellipsis",
    ],
}
