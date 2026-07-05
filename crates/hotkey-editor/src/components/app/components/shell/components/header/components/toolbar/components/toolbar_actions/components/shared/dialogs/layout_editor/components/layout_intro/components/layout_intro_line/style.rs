use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "m-0",
    "uppercase",
    "tracking-[0.1em]",
    "text-[2.1rem]/[1.35]",
    "text-warcraft-gold/85",
];

const MOBILE: &[TailwindClass] = tw![
    "mobile:text-[clamp(13px,3.5vw,16px)]",
    "mobile:tracking-[0.05em]",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:text-[clamp(13px,3.5vw,16px)]",
    "tablet:tracking-[0.05em]",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
