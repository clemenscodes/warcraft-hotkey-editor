use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "flex-col",
        "w-[80vw]",
        "h-[80vh]",
        "p-0",
        "gap-0",
        "overflow-hidden",
        "rounded-container",
        "border",
        "border-warcraft-gold",
        "bg-panel-toast",
        "shadow-overlay",
    ],
    mobile: tw![
        "mobile:w-screen",
        "mobile:h-dvh",
        "mobile:max-w-screen",
        "mobile:max-h-dvh",
        "mobile:rounded-none",
        "mobile:border-x-0",
    ],
    tablet: tw![
        "tablet:w-[90vw]",
        "tablet:h-[90vh]",
        "tablet:max-w-[90vw]",
        "tablet:max-h-[90vh]",
    ],
}
