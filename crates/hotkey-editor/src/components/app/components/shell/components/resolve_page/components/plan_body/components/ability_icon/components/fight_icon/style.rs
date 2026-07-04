use crate::{classes, styling::TailwindClass, tw};
const BASE: &[TailwindClass] = tw![
    "w-[72px]",
    "h-[72px]",
    "border",
    "border-[#2a5085]",
    "rounded-[7px]",
    "object-cover",
    "group-[:not(:disabled):hover]:border-warcraft-gold",
    "group-[:not(:disabled):hover]:[box-shadow:0_0_8px_rgba(255,206,99,0.5)]",
];
const MOBILE: &[TailwindClass] = tw![
    "mobile:w-[max(40px,min(72px,9vw))]",
    "mobile:h-[max(40px,min(72px,9vw))]",
];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
