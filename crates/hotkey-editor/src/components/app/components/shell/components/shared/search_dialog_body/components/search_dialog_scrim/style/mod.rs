use tw_macro::tw;

classes! {
    base: tw![
        "absolute",
        "inset-0",
        "z-20",
        "bg-warcraft-shadow/65",
        "cursor-pointer",
        "border-none",
        "p-0",
        "motion-safe:animate-fade-in",
        "motion-safe:aria-hidden:animate-fade-out",
        "aria-hidden:pointer-events-none",
    ],
    desktop: tw![
        "desktop:fixed",
        "desktop:z-60",
        "desktop:bg-transparent",
    ],
    qhd: tw![
        "qhd:fixed",
        "qhd:z-60",
        "qhd:bg-transparent",
    ],
    uhd: tw![
        "uhd:fixed",
        "uhd:z-60",
        "uhd:bg-transparent",
    ],
}
