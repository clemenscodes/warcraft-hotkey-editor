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
}
