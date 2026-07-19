use tw_macro::tw;
classes! {
    base: tw![
        "absolute",
        "left-[50%]",
        "top-[50%]",
        "transform-[translate(-50%,-50%)]",
        "flex-none",
        "text-warcraft-gold",
        "text-3xl",
        "leading-none",
        "text-shadow-drop",
    ],
    // With the grids stacked on a phone the arrow leaves the absolute centre,
    // sits in flow between them and rotates a quarter turn to point down.
    mobile: tw![
        "mobile:static",
        "mobile:transform-[rotate(90deg)]",
        "mobile:text-2xl",
    ],
}
