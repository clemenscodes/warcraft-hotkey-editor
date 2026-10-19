use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "grid",
        "grid-cols-[auto_minmax(0,1fr)]",
        // The figures sit at the top and the matchups at the bottom, so beside a
        // taller neighbour the room goes between them and both matchup blocks
        // line up across the two columns.
        "grid-rows-[auto_1fr]",
        "items-stretch",
        "gap-x-[1em]",
        "gap-y-[0.75em]",
        "min-w-0",
        "relative",
        "size-full",
    ],
    mobile: tw![
        "mobile:gap-x-[3.74cqi]",
        "mobile:gap-y-[1.2cqi]",
    ],
}
