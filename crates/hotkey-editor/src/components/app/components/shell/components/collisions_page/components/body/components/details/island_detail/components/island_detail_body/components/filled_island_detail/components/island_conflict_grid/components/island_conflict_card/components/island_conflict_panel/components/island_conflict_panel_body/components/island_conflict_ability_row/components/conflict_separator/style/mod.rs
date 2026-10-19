use tw_macro::tw;
classes! {
    base: tw![
        "flex-none",
        "h-18",
        "flex",
        "items-center",
        "text-warcraft-gold",
        "text-2xl",
        "leading-none",
        "text-shadow-drop",
        "[&_svg]:size-[1em]",
    ],
    // The icons are 64.3% of a 31cqi column (the 75.76cqi row less this 8cqi
    // separator and two 2.88cqi gaps, halved), so the cross centres on them.
    mobile: tw![
        "mobile:w-[8cqi]",
        "mobile:h-[19.93cqi]",
        "mobile:justify-center",
        "mobile:text-[2em]",
    ],
    tablet: tw![
        "tablet:w-[8cqi]",
        "tablet:h-[19.93cqi]",
        "tablet:justify-center",
        "tablet:text-[2em]",
    ],
    laptop: tw![
        "laptop:w-[8cqi]",
        "laptop:h-[19.93cqi]",
        "laptop:justify-center",
        "laptop:text-[2em]",
    ],
}
