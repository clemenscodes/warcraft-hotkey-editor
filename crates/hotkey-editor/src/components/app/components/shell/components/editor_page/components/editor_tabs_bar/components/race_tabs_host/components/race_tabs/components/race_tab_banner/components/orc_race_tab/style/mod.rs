use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "flex-1",
        "min-w-0",
        "[--race-color:var(--color-race-orc)]",
        "[--banner-strength:25%]",
        "[--banner-image:url('/warcraft-hotkey-editor/webui/common/dark-banner-orc.png')]",
    ],
    mobile: tw![
        "mobile:flex-[0_0_40cqi]",
        "mobile:snap-start",
    ],
}
