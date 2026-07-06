use tw_macro::tw;
// The brand's box, and the query context every `cqi` length inside the brand resolves
// against. Its width is definite per band, never `flex-auto` (which grabs all free space
// and blows the wordmark up on tablet). It is always a fraction of the header bar measured
// in `cqi` (the header is the container on every band now), so the brand scales purely with
// the bar as one drawing — no fixed floor or cap, and the raw viewport never enters. `26cqi`
// is the laptop-and-up default, held in BASE; the two touch bands override it with their
// own wider `cqi` fractions.

classes! {
    base: tw![
        "@container",
        "min-w-0",
        "w-[26cqi]",
    ],
    mobile: tw!["mobile:w-[72cqi]"],
    tablet: tw!["tablet:w-[55cqi]"],
}
