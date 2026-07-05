use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "absolute",
    "inset-0",
    "flex",
    "items-center",
    "justify-center",
    "px-[4%]",
    "text-[13cqi]/[1.1]",
    "text-center",
    "uppercase",
    "tracking-[0.04em]",
    "text-warcraft-gold",
    "[text-shadow:1px_1px_0_#000,-1px_1px_0_#000,1px_-1px_0_#000,-1px_-1px_0_#000]",
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
