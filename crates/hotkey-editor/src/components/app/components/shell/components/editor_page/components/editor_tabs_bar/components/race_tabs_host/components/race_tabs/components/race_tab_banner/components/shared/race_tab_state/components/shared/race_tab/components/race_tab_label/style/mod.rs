use tw_macro::tw;

classes! {
    base: tw![
        "relative",
        "z-2",
        "pt-[2.38cqi]",
        "px-[3.96cqi]",
        "pb-[3.17cqi]",
        "w-full",
        "min-w-0",
        "text-(--label-color,var(--color-white))",
    ],
    mobile: tw![
        "mobile:pt-[5.87cqi]",
        "mobile:px-[2.93cqi]",
        "mobile:pb-[11.73cqi]",
        "mobile:text-xs",
        "mobile:tracking-snug",
    ],
    tablet: tw![
        "tablet:pt-[3.09cqi]",
        "tablet:px-[1.55cqi]",
        "tablet:pb-[6.18cqi]",
        "tablet:text-xs",
        "tablet:tracking-snug",
    ],
}
