use crate::{classes, styling::TailwindClass, tw};
const BASE: &[TailwindClass] = tw![
    "m-0",
    "uppercase",
    "tracking-[0.12em]",
    "text-warcraft-gold",
    "[text-shadow:1px_1px_0_var(--color-warcraft-shadow)]",
];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
