use tw_macro::tw;

// The description flows around the portrait on a phone. The room between them
// is the slot's own padding, counted into its width, so the portrait keeps its
// size.
classes! {
    base: tw![
        "flex-none",
        "w-[6em]",
    ],
    mobile: tw![
        "mobile:float-left",
        "mobile:w-[28.66cqi]",
        "mobile:pr-[4.78cqi]",
        "mobile:pb-[1.49cqi]",
    ],
}
