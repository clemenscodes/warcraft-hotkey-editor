use tw_macro::tw;
// Every panel spans two tracks of a grid twice as fine as its columns, so a
// last row that is not full starts further in and sits centred, the panels
// keeping their size.
classes! {
    base: tw![
        "grid",
        "grid-cols-[repeat(auto-fill,minmax(450px,1fr))]",
        "gap-6",
        "flex-[1_1_0]",
        "min-h-0",
        "overflow-y-auto",
        "content-start",
        "pt-4",
        "pr-3",
        "pb-4",
        "pl-0",
    ],
    mobile: tw![
        "mobile:grid-cols-[minmax(0,1fr)]",
        "mobile:flex-none",
        "mobile:min-h-auto",
        "mobile:overflow-y-visible",
        "mobile:py-[1em]",
        "mobile:px-0",
        "mobile:gap-[1.5em]",
    ],
    tablet: tw![
        "tablet:grid-cols-4",
        "tablet:*:col-span-2",
        "tablet:*:odd:last:col-start-2",
        "tablet:flex-none",
        "tablet:min-h-auto",
        "tablet:overflow-y-visible",
        "tablet:py-[1em]",
        "tablet:px-0",
        "tablet:gap-[1.5em]",
    ],
    laptop: tw![
        "laptop:grid-cols-6",
        "laptop:*:col-span-2",
        "laptop:*:nth-[3n+1]:last:col-start-3",
        "laptop:*:nth-[3n+1]:nth-last-2:col-start-2",
        "laptop:flex-none",
        "laptop:min-h-auto",
        "laptop:overflow-y-visible",
        "laptop:py-[1em]",
        "laptop:px-0",
        "laptop:gap-[1.5em]",
    ],
    desktop: tw![
        "desktop:grid-cols-4",
        "desktop:*:col-span-2",
        "desktop:*:odd:last:col-start-2",
    ],
    qhd: tw![
        "qhd:grid-cols-6",
        "qhd:*:col-span-2",
        "qhd:*:nth-[3n+1]:last:col-start-3",
        "qhd:*:nth-[3n+1]:nth-last-2:col-start-2",
    ],
    uhd: tw![
        "uhd:grid-cols-8",
        "uhd:*:col-span-2",
        "uhd:*:nth-[4n+1]:last:col-start-4",
        "uhd:*:nth-[4n+1]:nth-last-2:col-start-3",
        "uhd:*:nth-[4n+1]:nth-last-3:col-start-2",
    ],
}
