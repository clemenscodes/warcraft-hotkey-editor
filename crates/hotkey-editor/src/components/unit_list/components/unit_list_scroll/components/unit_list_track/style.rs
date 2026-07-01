use crate::classes;

// The list of category sections and cards. A vertical stack on the sidebar; a
// horizontal fixed-height row (the swipe carousel) on small screens.
const BASE: &[&str] = &["flex", "flex-col", "gap-2"];

const MOBILE: &[&str] = &[
    "mobile:flex-row",
    "mobile:flex-nowrap",
    "mobile:w-max",
    "mobile:min-w-full",
    "mobile:items-stretch",
    "mobile:h-[clamp(96px,25vw,120px)]",
    "mobile:min-h-[clamp(96px,25vw,120px)]",
    "mobile:max-h-[clamp(96px,25vw,120px)]",
    "mobile:px-[0.4rem]",
    "mobile:py-0",
];

const TABLET: &[&str] = &[
    "tablet:flex-row",
    "tablet:flex-nowrap",
    "tablet:w-max",
    "tablet:min-w-full",
    "tablet:items-stretch",
    "tablet:h-[clamp(96px,25vw,120px)]",
    "tablet:min-h-[clamp(96px,25vw,120px)]",
    "tablet:max-h-[clamp(96px,25vw,120px)]",
    "tablet:px-[0.4rem]",
    "tablet:py-0",
];

const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
