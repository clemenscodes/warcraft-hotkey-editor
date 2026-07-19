use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "relative",
        "flex",
        "items-center",
        "justify-center",
        "gap-[2.69cqi]",
        "w-full",
    ],
    // On a phone the two grids stack vertically and the transition grows to
    // fill the card, so the before and after grids each get the full width and
    // the arrow sits between them pointing down instead of overflowing.
    mobile: tw![
        "mobile:flex-1",
        "mobile:flex-col",
        "mobile:justify-center",
        "mobile:gap-4",
    ],
}
