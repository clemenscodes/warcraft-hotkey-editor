use tw_macro::tw;
// The dialog content box: the bordered, self-centring surface that holds the frame's
// regions. The headless `Dialog` applies this class to its content container via `class:`;
// the primitive owns no positioning, so the box places itself (fixed + inset-0 + m-auto
// centres the sized box on the viewport).

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
