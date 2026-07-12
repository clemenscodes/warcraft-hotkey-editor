use tw_macro::tw;
// The isolated preview content's own box: the scrolling region that fills the space it is
// given — a dialog's content box below the header, or a bare page — holding the read-only
// textarea. The padding and scroll live here; the textarea flexes to fill it.

classes! {
    base: tw![
        "flex-1",
        "min-h-0",
        "flex",
        "flex-col",
        "gap-6",
        "pt-10",
        "px-12",
        "pb-10",
        "overflow-y-auto",
    ],
    mobile: tw![
        "mobile:pt-5",
        "mobile:px-4",
        "mobile:pb-6",
    ],
}
