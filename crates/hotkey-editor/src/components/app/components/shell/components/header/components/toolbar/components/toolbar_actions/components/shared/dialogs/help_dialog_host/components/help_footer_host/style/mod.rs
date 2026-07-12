use tw_macro::tw;
// The pinned footer bar below the scrolling guide body: it does not scroll (`flex-none`),
// sits below the body separated by a gold rule (`border-t`), and right-aligns the dismiss
// button. Mobile and tablet centre the button and tighten the horizontal padding.

classes! {
    base: tw![
        "flex",
        "items-center",
        "justify-end",
        "flex-none",
        "gap-4",
        "pt-6",
        "px-18",
        "pb-7",
        "border-t",
        "border-warcraft-gold/40",
    ],
    mobile: tw![
        "mobile:justify-center",
        "mobile:px-6",
    ],
    tablet: tw![
        "tablet:justify-center",
    ],
}
