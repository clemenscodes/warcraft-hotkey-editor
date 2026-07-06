use tw_macro::tw;
// 44% of the button box (a cqi fraction), matching the shared toolbar button, so the
// glyph scales with the button instead of staying a fixed size.

classes! {
    base: tw![
        "flex",
        "items-center",
        "justify-center",
        "leading-none",
        "w-[44cqi]",
        "h-[44cqi]",
        "[&_svg]:block",
        "[&_svg]:w-full",
        "[&_svg]:h-full",
    ],
}
