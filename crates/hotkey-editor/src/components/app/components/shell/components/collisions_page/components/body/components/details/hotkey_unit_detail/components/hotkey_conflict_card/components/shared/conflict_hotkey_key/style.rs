use tw_macro::tw;
classes! {
    base: tw![
        "box-border",
        "min-w-[60px]",
        "h-[60px]",
        "px-[11px]",
        "inline-flex",
        "items-center",
        "justify-center",
        "text-2xl",
        "leading-none",
        "text-warcraft-gold",
        "bg-warcraft-gold/12",
        "border-2",
        "border-warcraft-gold",
        "rounded-card",
        "text-shadow-drop",
    ],
    mobile: tw![
        "mobile:w-[56px]",
        "mobile:min-w-0",
        "mobile:h-[56px]",
        "mobile:p-0",
        "mobile:text-xl",
    ],
    tablet: tw![
        "tablet:w-[56px]",
        "tablet:min-w-0",
        "tablet:h-[56px]",
        "tablet:p-0",
        "tablet:text-xl",
    ],
}
