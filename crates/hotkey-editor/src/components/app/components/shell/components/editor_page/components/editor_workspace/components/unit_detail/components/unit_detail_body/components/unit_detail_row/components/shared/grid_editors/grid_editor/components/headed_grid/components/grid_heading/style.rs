use tw_macro::tw;
classes! {
    base: tw![
        "mx-0",
        "text-md",
        "font-normal",
        "uppercase",
        "tracking-[0.08em]",
        "text-warcraft-gold",
        "text-shadow-drop",
    ],
    mobile: tw!["mobile:text-base"],
    tablet: tw!["tablet:text-md"],
    desktop: tw!["desktop:text-lg"],
    qhd: tw!["qhd:text-xl"],
    uhd: tw!["uhd:text-2xl"],
}
