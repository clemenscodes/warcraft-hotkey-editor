use tw_macro::tw;
// The layout button's box and container-query context. It owns the centered column at
// laptop width and up (hidden below, where the drawer offers the action instead). The
// header hands it the shared row height, which it takes explicitly with `h-full`, and
// `aspect-[39/10]` turns that height into a definite width — so the box is sized entirely
// by the bar, with no fixed length of its own. `h-full` (not the bar's `items-stretch`
// alone) is load-bearing: this box lives in the grid's `auto` middle track, and Firefox
// only derives the aspect-ratio width for that track from a *definite* height. A stretched
// height is indefinite at track-sizing time, so Firefox collapses the column to zero width
// and the button becomes unclickable; the explicit `h-full` gives it the definite height
// it needs. It marks itself the query container off that definite width, so the button
// fills it (`size-full`) and every `cqi` length inside — padding, gap, border, radius,
// font, icon — resolves against this box and scales as one drawing with the bar.

classes! {
    base: tw![
        "@container",
        "hidden",
        "items-center",
        "justify-center",
        "h-full",
        "aspect-39/10",
    ],
    laptop: tw![
        "laptop:flex",
    ],
    desktop: tw![
        "desktop:flex",
    ],
    qhd: tw![
        "qhd:flex",
    ],
    uhd: tw![
        "uhd:flex",
    ],
}
