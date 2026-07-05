use crate::{classes, styling::TailwindClass, tw};
const BASE: &[TailwindClass] = tw![
    "w-[72px]",
    "h-[72px]",
    "border",
    "border-warcraft-blue",
    "rounded-[7px]",
    "object-cover",
    "group-[:not(:disabled):hover]:border-warcraft-gold",
    "group-[:not(:disabled):hover]:shadow-glow-8",
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
