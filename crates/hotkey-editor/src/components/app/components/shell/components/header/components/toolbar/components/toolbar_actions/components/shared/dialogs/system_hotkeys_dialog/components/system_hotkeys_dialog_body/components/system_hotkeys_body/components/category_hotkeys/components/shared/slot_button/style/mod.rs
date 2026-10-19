use tw_macro::tw;

classes! {
    base: tw![
        "relative",
        "group/tooltip",
        "[anchor-name:--tooltip-anchor]",
        "[anchor-scope:--tooltip-anchor]",
        "appearance-none",
        "border-0",
        "bg-transparent",
        "p-0",
        "cursor-pointer",
        "touch-manipulation",
        "focus:outline-none",
        "kb-focus:outline-none",
        "min-h-(--slot-host-min-h,auto)",
    ],
    mobile: tw![
        "mobile:aspect-(--slot-host-aspect,1/0.95)",
        "mobile:min-h-(--slot-host-min-h,--spacing(19))",
    ],
    tablet: tw![
        "tablet:aspect-(--slot-host-aspect,1/0.95)",
        "tablet:min-h-(--slot-host-min-h,--spacing(19))",
    ],
    laptop: tw![
        "laptop:aspect-(--slot-host-aspect,1/0.95)",
        "laptop:min-h-(--slot-host-min-h,--spacing(19))",
    ],
    desktop: tw![
        "desktop:aspect-(--slot-host-aspect,1/0.95)",
        "desktop:min-h-(--slot-host-min-h,--spacing(19))",
    ],
    qhd: tw![
        "qhd:aspect-(--slot-host-aspect,1/0.95)",
        "qhd:min-h-(--slot-host-min-h,--spacing(19))",
    ],
    uhd: tw![
        "uhd:aspect-(--slot-host-aspect,1/0.95)",
        "uhd:min-h-(--slot-host-min-h,--spacing(19))",
    ],
}
