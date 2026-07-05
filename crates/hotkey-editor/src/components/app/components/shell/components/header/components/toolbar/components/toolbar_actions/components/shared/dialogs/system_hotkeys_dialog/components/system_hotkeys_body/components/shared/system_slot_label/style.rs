use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "uppercase",
    "tracking-[0.18em]",
    "text-[2.4rem]",
    "leading-none",
    "text-warcraft-gold/55",
    "text-shadow-drop",
];

const MOBILE: &[TailwindClass] = tw![
    "mobile:text-[clamp(10px,2.6vw,12px)]",
    "mobile:tracking-[0.08em]",
    "mobile:data-[compact=true]:text-[clamp(9px,2.2vw,11px)]",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:text-[clamp(10px,2.6vw,12px)]",
    "tablet:tracking-[0.08em]",
    "tablet:data-[compact=true]:text-[clamp(9px,2.2vw,11px)]",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
