use crate::classes;

// The workspace is a grid: a single stacked column on mobile and tablet (the unit
// list sits above the detail panel, both in flow), and a two-column sidebar-plus-
// detail layout from laptop up (the unit list is absolutely positioned over the
// first column by its own bands, so only the detail panel flows). The sidebar column
// widths match the unit list's own per-band widths.
const BASE: &[&str] = &[
    "relative",
    "grid",
    "grid-cols-[minmax(0,1fr)]",
    "flex-[1_1_0]",
    "items-stretch",
    "min-h-0",
    "gap-[1rem]",
    "mt-[0.5rem]",
];
const MOBILE: &[&str] = &["mobile:flex-none"];
const TABLET: &[&str] = &["tablet:flex-none"];
const LAPTOP: &[&str] = &[
    "laptop:grid-cols-[34rem_minmax(0,1fr)]",
    "laptop:[grid-template-rows:1fr]",
    "laptop:gap-[2.5rem]",
    "laptop:mt-[1.5rem]",
    "laptop:overflow-hidden",
];
const DESKTOP: &[&str] = &[
    "desktop:grid-cols-[34rem_minmax(0,1fr)]",
    "desktop:[grid-template-rows:1fr]",
    "desktop:gap-[2.5rem]",
    "desktop:mt-[1.5rem]",
    "desktop:overflow-hidden",
];
const QHD: &[&str] = &[
    "qhd:grid-cols-[46rem_minmax(0,1fr)]",
    "qhd:[grid-template-rows:1fr]",
    "qhd:gap-[2.5rem]",
    "qhd:mt-[1.5rem]",
    "qhd:overflow-hidden",
];
const UHD: &[&str] = &[
    "uhd:grid-cols-[62rem_minmax(0,1fr)]",
    "uhd:[grid-template-rows:1fr]",
    "uhd:gap-[2.5rem]",
    "uhd:mt-[1.5rem]",
    "uhd:overflow-hidden",
];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
