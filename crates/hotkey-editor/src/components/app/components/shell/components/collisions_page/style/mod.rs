use tw_macro::tw;
// From a desktop up the page is the editor's grid: the collision list down the
// first column, the kind tabs over the detail in the second.
classes! {
    base: tw![
        "flex",
        "flex-col",
        "gap-6",
        "flex-[1_1_0]",
        "min-h-0",
    ],
    desktop: tw![
        "desktop:p-4",
        "desktop:grid",
        "desktop:grid-cols-[43rem_minmax(0,1fr)]",
        "desktop:grid-rows-[auto_minmax(0,1fr)]",
        "desktop:gap-x-5",
        "desktop:gap-y-4",
        "desktop:*:first:col-start-2",
        "desktop:*:first:row-start-1",
        "desktop:*:nth-2:col-span-full",
    ],
    qhd: tw![
        "qhd:p-4",
        "qhd:grid",
        "qhd:grid-cols-[64.5rem_minmax(0,1fr)]",
        "qhd:grid-rows-[auto_minmax(0,1fr)]",
        "qhd:gap-x-5",
        "qhd:gap-y-4",
        "qhd:*:first:col-start-2",
        "qhd:*:first:row-start-1",
        "qhd:*:nth-2:col-span-full",
    ],
    uhd: tw![
        "uhd:p-4",
        "uhd:grid",
        "uhd:grid-cols-[86rem_minmax(0,1fr)]",
        "uhd:grid-rows-[auto_minmax(0,1fr)]",
        "uhd:gap-x-5",
        "uhd:gap-y-4",
        "uhd:*:first:col-start-2",
        "uhd:*:first:row-start-1",
        "uhd:*:nth-2:col-span-full",
    ],
}
