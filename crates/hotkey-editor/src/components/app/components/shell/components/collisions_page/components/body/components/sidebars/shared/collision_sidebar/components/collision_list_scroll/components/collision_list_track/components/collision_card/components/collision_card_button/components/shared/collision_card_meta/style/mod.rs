use tw_macro::tw;

// From a desktop up the list shares the unit list's columns, so its cards carry
// two lines of text like a unit card: the name or position across the top, the
// id and the count side by side under it. The id's column has a fixed width,
// so the count starts at the same place on every card; a position card has no
// id, and its count runs across both columns.
classes! {
    base: tw![
        "flex",
        "flex-col",
        "gap-2",
        "min-w-0",
        "flex-auto",
        "py-1.5",
        "px-2",
    ],
    desktop: tw![
        "desktop:grid",
        "desktop:grid-cols-[3.5rem_minmax(0,1fr)]",
        "desktop:justify-start",
        "desktop:items-baseline",
        "desktop:gap-x-3",
        "desktop:gap-y-0.5",
        "desktop:py-0",
        "desktop:px-0",
        "desktop:*:first:col-span-full",
        "desktop:*:nth-2:last:col-span-full",
    ],
    qhd: tw![
        "qhd:grid",
        "qhd:grid-cols-[3.5rem_minmax(0,1fr)]",
        "qhd:justify-start",
        "qhd:items-baseline",
        "qhd:gap-x-3",
        "qhd:gap-y-0.5",
        "qhd:py-0",
        "qhd:px-0",
        "qhd:*:first:col-span-full",
        "qhd:*:nth-2:last:col-span-full",
    ],
    uhd: tw![
        "uhd:grid",
        "uhd:grid-cols-[3.5rem_minmax(0,1fr)]",
        "uhd:justify-start",
        "uhd:items-baseline",
        "uhd:gap-x-3",
        "uhd:gap-y-0.5",
        "uhd:py-0",
        "uhd:px-0",
        "uhd:*:first:col-span-full",
        "uhd:*:nth-2:last:col-span-full",
    ],
}
