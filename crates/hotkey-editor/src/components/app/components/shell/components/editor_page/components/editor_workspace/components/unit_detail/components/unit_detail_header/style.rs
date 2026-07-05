use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "grid",
    "grid-cols-[clamp(5.75rem,4.8vw,7.5rem)_1fr]",
    "items-start",
    "gap-x-[clamp(0.85rem,0.8vw,1.25rem)]",
    "mb-0",
    "pb-[clamp(0.4rem,0.7vh,0.7rem)]",
    "border-b",
    "border-warcraft-blue-deep",
];
const MOBILE: &[TailwindClass] = tw![
    "mobile:flex",
    "mobile:flex-row",
    "mobile:items-start",
    "mobile:h-auto",
    "mobile:min-h-0",
    "mobile:gap-[12px]",
    "mobile:pb-[16px]",
    "mobile:overflow-visible",
    "mobile:text-left",
    "mobile:w-full",
    "mobile:min-w-0",
];
const TABLET: &[TailwindClass] = tw![
    "tablet:flex",
    "tablet:flex-row",
    "tablet:items-start",
    "tablet:h-auto",
    "tablet:min-h-0",
    "tablet:gap-[12px]",
    "tablet:pb-[16px]",
    "tablet:overflow-visible",
    "tablet:text-left",
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
