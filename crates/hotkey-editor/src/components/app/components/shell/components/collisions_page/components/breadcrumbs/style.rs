use crate::{classes, styling::TailwindClass, tw};
const BASE: &[TailwindClass] = tw![
    "flex",
    "items-center",
    "gap-3",
    "flex-none",
    "py-[1.2rem]",
    "border-b",
    "border-warcraft-gold/25",
];
const MOBILE: &[TailwindClass] = tw![
    "mobile:gap-[0.85rem]",
    "mobile:flex-nowrap",
    "mobile:overflow-x-auto",
    "mobile:overflow-y-hidden",
    "mobile:overscroll-x-contain",
    "mobile:snap-x",
    "mobile:snap-mandatory",
    "mobile:[scrollbar-width:none]",
    "mobile:[&::-webkit-scrollbar]:hidden",
];
const TABLET: &[TailwindClass] = tw![
    "tablet:gap-[0.85rem]",
    "tablet:flex-nowrap",
    "tablet:overflow-x-auto",
    "tablet:overflow-y-hidden",
    "tablet:overscroll-x-contain",
    "tablet:snap-x",
    "tablet:snap-mandatory",
    "tablet:[scrollbar-width:none]",
    "tablet:[&::-webkit-scrollbar]:hidden",
];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
