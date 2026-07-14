use tw_macro::tw;

classes! {
    base: tw![
        "w-full",
    ],
    mobile: tw![
        "mobile:flex-[1_0_auto]",
        "mobile:w-[min(54cqi,260px)]",
        "mobile:h-full",
        "mobile:snap-start",
    ],
    tablet: tw![
        "tablet:flex-[1_0_auto]",
        "tablet:w-[min(54cqi,260px)]",
        "tablet:h-full",
        "tablet:snap-start",
    ],
}
