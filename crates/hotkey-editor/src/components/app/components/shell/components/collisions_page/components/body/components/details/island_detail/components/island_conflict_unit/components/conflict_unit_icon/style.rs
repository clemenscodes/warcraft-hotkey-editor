use crate::{classes, styling::TailwindClass, tw};
const BASE: &[TailwindClass] = tw![
    "w-[112px]",
    "h-[112px]",
    "border",
    "border-warcraft-blue",
    "rounded-[6px]",
    "object-cover",
    "hover:border-warcraft-gold",
    "hover:shadow-[0_0_8px_color-mix(in_oklab,var(--color-warcraft-gold)_50%,transparent)]",
];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
