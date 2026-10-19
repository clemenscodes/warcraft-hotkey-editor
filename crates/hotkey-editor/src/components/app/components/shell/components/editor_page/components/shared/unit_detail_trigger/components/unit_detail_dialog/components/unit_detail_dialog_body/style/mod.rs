use tw_macro::tw;
// The body sets the dialog's type size per band. Everything inside sizes in
// em from it. From a desktop up the type follows the dialog, its width on a
// narrow window and the window height on a wide one, so the stats fill the
// panel without ever outgrowing its four fifths of the window.
classes! {
    base: tw![
        "@container",
        "flex-1",
        "min-h-0",
        "flex",
        "flex-col",
        "gap-[1em]",
        "p-[1em]",
        "leading-heading",
        "overflow-y-auto",
        "overflow-x-clip",
        "overscroll-contain",
    ],
    mobile: tw![
        "mobile:text-sm",
        "mobile:gap-2",
        "mobile:p-3",
    ],
    tablet: tw![
        "tablet:text-md",
    ],
    laptop: tw![
        "laptop:text-md",
    ],
    desktop: tw![
        "desktop:*:text-[1.6cqw]",
        "desktop:[@media(min-aspect-ratio:16/10)]:*:text-[2.15vh]",
    ],
    qhd: tw![
        "qhd:*:text-[1.6cqw]",
        "qhd:[@media(min-aspect-ratio:16/10)]:*:text-[2.15vh]",
    ],
    uhd: tw![
        "uhd:*:text-[1.6cqw]",
        "uhd:[@media(min-aspect-ratio:2/1)]:*:text-[2.15vh]",
    ],
}
