use tw_macro::tw;

classes! {
    base: tw![
        "w-full",
    ],
    tablet: tw![
        "tablet:flex-[1_0_auto]",
        "tablet:w-[min(54cqi,260px)]",
        "tablet:h-full",
        "tablet:@container-size",
        "tablet:snap-start",
    ],
}
