use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "justify-center",
        "gap-1",
    ],
    // A keyboard row is wider than a phone. Rather than shrink the keys until
    // their labels no longer fit, let the row break onto as many lines as it
    // needs and keep the keys at a legible size.
    mobile: tw![
        "mobile:gap-0.5",
        "mobile:flex-wrap",
        // Centred by default so the twelve-key board keeps its keyboard taper.
        // A board whose rows actually wrap sets this to flex-start, so the
        // continuation lines start at the edge instead of floating mid-dialog.
        "mobile:[justify-content:var(--key-row-justify,center)]",
    ],
}
