use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "flex",
    "items-center",
    "justify-between",
    "gap-[clamp(0.75rem,0.8vw,1rem)]",
    "min-h-0",
];
const MOBILE: &[TailwindClass] = tw![
    "mobile:flex-row",
    "mobile:items-center",
    "mobile:gap-[8px]",
    "mobile:w-full",
    "mobile:min-w-0",
];
const TABLET: &[TailwindClass] = tw![
    "tablet:flex-row",
    "tablet:items-center",
    "tablet:gap-[8px]",
    "tablet:w-full",
    "tablet:min-w-0",
];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
