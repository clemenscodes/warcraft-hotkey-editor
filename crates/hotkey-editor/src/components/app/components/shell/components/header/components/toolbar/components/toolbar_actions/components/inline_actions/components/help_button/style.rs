use tw_macro::tw;
// The button's slot in the inline row: shown from laptop up (the burger replaces the
// inline row below). It fills the shared row height and centers the button. The slot is
// display:none below laptop, but the row it sits in stays mounted at every width — so a
// button that also renders a dialog (a fixed overlay, a sibling of this slot, not inside
// it) keeps that dialog mounted on mobile, where this slot is hidden and the burger flips
// the shared open signal.

classes! {
    base: tw![
        "hidden",
        "items-center",
        "justify-center",
        "h-full",
    ],
    laptop: tw!["laptop:flex"],
    desktop: tw!["desktop:flex"],
    qhd: tw!["qhd:flex"],
    uhd: tw!["uhd:flex"],
}
