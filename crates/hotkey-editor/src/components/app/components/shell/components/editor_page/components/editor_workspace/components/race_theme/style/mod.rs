use tw_macro::tw;

classes! {
    base: tw![
        "contents",
        "has-[.human-race-theme]:[--race-color:var(--color-race-human)]",
        "has-[.orc-race-theme]:[--race-color:var(--color-race-orc)]",
        "has-[.nightelf-race-theme]:[--race-color:var(--color-race-nightelf)]",
        "has-[.undead-race-theme]:[--race-color:var(--color-race-undead)]",
        "has-[.neutral-race-theme]:[--race-color:var(--color-warcraft-gold)]",
    ],
}
