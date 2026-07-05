use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "uppercase",
    "tracking-[0.04em]",
    "text-[3.4rem]",
    "leading-none",
    "whitespace-nowrap",
    "text-warcraft-gold",
    "[text-shadow:1px_1px_0_#000,0_0_14px_rgba(255,206,99,0.45)]",
];

const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
