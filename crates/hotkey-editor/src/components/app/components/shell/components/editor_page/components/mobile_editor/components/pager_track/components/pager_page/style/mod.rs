use tw_macro::tw;
// One page of the pager, the box its motion measures and moves. A laptop shows
// two units side by side; a page with a single unit left over centres it at
// the width it would have beside a second one.
classes! {
    base: tw![
        "grid",
        "grid-cols-1",
        "min-w-0",
        "shrink-0",
    ],
    tablet: tw![
        "tablet:w-full",
    ],
    laptop: tw![
        "laptop:grid-cols-4",
        "laptop:gap-4",
        "laptop:*:col-span-2",
        "laptop:*:only:col-start-2",
    ],
}
