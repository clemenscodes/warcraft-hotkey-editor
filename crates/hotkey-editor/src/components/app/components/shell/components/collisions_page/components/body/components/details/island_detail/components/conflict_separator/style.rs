use crate::{classes, styling::TailwindClass, tw};
const BASE: &[TailwindClass] = tw![
    "flex-none",
    "h-[72px]",
    "flex",
    "items-center",
    "text-warcraft-gold",
    "text-[34px]",
    "leading-none",
    "[text-shadow:1px_1px_0_var(--color-warcraft-shadow)]",
];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
