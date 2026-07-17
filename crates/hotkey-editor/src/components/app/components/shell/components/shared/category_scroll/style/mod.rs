use tw_macro::tw;

// The horizontal rail is the aside's tablet layout, not the list's nature: the
// base is a plain vertical list and the tablet band turns it into a rail. There
// is no mobile band because below 768px the editor page mounts the pager instead
// of the aside, so the only list rendering there is the search dialog's — and a
// dialog wants the vertical list.
classes! {
    base: tw![
        "flex-1",
        "flex",
        "flex-col",
        "min-h-0",
        "overflow-y-auto",
        "overflow-x-hidden",
        "pr-1",
    ],
    tablet: tw![
        "tablet:overflow-x-auto",
        "tablet:overflow-y-hidden",
        "tablet:max-h-none",
        "tablet:pr-0",
        "tablet:pb-1",
        "tablet:flex-none",
        "tablet:@container",
        "tablet:h-30",
        "tablet:[-webkit-overflow-scrolling:touch]",
        "tablet:overscroll-x-contain",
        "tablet:snap-x",
        "tablet:snap-mandatory",
        "tablet:scroll-ps-[0.4rem]",
    ],
}
