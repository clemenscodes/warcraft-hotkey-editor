use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "text-[1.5rem]",
    "text-warcraft-gold",
    "[text-shadow:1px_1px_0_#000]",
    "whitespace-nowrap",
    "flex-none",
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
