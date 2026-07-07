use tw_macro::tw;
// The unit's flavor text under the header. Reserves two lines' height on desktop so
// the stats card below never shifts; clamps to a single line on smaller panels.

classes! {
    base: tw![
        "mt-4",
        "min-h-[9rem]",
        "text-xl",
        "leading-body",
        "text-warcraft-text-secondary",
        "text-shadow-drop-60",
    ],
    mobile: tw![
        "mobile:mt-3",
        "mobile:flex-none",
        "mobile:h-[1.4em]",
        "mobile:min-h-[1.4em]",
        "mobile:max-h-[1.4em]",
        "mobile:max-w-full",
        "mobile:text-sm",
        "mobile:leading-body",
        "mobile:line-clamp-1",
        "mobile:[overflow-wrap:break-word]",
        "mobile:[word-break:break-word]",
    ],
    tablet: tw![
        "tablet:mt-3",
        "tablet:flex-none",
        "tablet:h-[1.4em]",
        "tablet:min-h-[1.4em]",
        "tablet:max-h-[1.4em]",
        "tablet:max-w-full",
        "tablet:text-sm",
        "tablet:leading-body",
        "tablet:line-clamp-1",
        "tablet:[overflow-wrap:break-word]",
        "tablet:[word-break:break-word]",
    ],
}
