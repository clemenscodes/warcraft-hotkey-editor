use tw_macro::tw;

classes! {
    base: tw![
        "fixed",
        "inset-0",
        "z-1000",
        "m-auto",
        "flex",
        "flex-col",
        "w-[80vw]",
        "h-[80vh]",
        "overflow-hidden",
        "rounded-container",
        "border",
        "border-warcraft-gold",
        "bg-warcraft-bg-base",
        "bg-panel-toast",
        "shadow-overlay",
    ],
    mobile: tw![
        "mobile:w-screen",
        "mobile:h-dvh",
        "mobile:rounded-none",
        "mobile:border-x-0",
    ],
    tablet: tw![
        "tablet:w-[90vw]",
        "tablet:h-[90vh]",
    ],
}
