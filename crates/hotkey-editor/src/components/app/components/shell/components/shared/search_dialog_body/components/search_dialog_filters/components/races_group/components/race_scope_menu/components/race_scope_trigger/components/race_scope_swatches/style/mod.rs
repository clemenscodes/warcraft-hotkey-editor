use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "gap-1",
        "shrink-0",
        "[&>span]:w-3",
        "[&>span]:h-3",
        "[&>span]:rounded-hairline",
        "[&>span]:border",
        "[&>span]:border-warcraft-shadow/40",
        "[&>span:nth-child(1)]:bg-race-human",
        "[&>span:nth-child(2)]:bg-race-orc",
        "[&>span:nth-child(3)]:bg-race-nightelf",
        "[&>span:nth-child(4)]:bg-race-undead",
        "[&>span:nth-child(5)]:bg-race-neutral",
    ],
    desktop: tw![
        "desktop:gap-1.25",
        "desktop:[&>span]:w-3.5",
        "desktop:[&>span]:h-3.5",
    ],
    qhd: tw![
        "qhd:gap-1.75",
        "qhd:[&>span]:w-5.25",
        "qhd:[&>span]:h-5.25",
    ],
    uhd: tw![
        "uhd:gap-1.75",
        "uhd:[&>span]:w-5.5",
        "uhd:[&>span]:h-5.5",
    ],
}
