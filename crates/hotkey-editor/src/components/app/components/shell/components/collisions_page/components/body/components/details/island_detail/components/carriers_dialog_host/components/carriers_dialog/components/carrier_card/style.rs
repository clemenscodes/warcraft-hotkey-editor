use crate::{classes, styling::TailwindClass, tw};
const BASE: &[TailwindClass] = tw![
    "flex",
    "flex-col",
    "items-center",
    "gap-[18px]",
    "m-0",
    "p-[30px_20px]",
    "bg-[rgba(13,31,61,0.5)]",
    "border",
    "border-[#24406a]",
    "rounded-[8px]",
    "cursor-pointer",
    "hover:border-warcraft-gold",
    "hover:shadow-[0_0_8px_rgba(255,206,99,0.45)]",
];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
