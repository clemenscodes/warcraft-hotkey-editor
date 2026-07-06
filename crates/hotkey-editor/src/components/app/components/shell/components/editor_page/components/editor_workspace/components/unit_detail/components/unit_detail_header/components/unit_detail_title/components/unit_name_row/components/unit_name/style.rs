use tw_macro::tw;
classes! {
    base: tw![
        "text-[clamp(2.2rem,0.85vw+1.1rem,3rem)]/[1.1]",
        "font-normal",
        "text-warcraft-gold",
        "text-shadow-drop-92",
        "m-0",
        "tracking-[0.03em]",
    ],
    mobile: tw![
        "mobile:flex-auto",
        "mobile:min-w-0",
        "mobile:text-[clamp(17px,4.8vw,22px)]",
        "mobile:leading-[1.2]",
        "mobile:text-left",
        "mobile:[overflow-wrap:break-word]",
        "mobile:[word-break:break-word]",
    ],
    tablet: tw![
        "tablet:flex-auto",
        "tablet:min-w-0",
        "tablet:text-[clamp(17px,4.8vw,22px)]",
        "tablet:leading-[1.2]",
        "tablet:text-left",
        "tablet:[overflow-wrap:break-word]",
        "tablet:[word-break:break-word]",
    ],
}
