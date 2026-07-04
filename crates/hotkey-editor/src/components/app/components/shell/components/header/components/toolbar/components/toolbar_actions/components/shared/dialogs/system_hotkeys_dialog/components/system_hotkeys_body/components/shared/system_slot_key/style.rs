use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "font-friz-quadrata",
    "uppercase",
    "tracking-[0.04em]",
    "text-[3.4rem]",
    "leading-none",
    "whitespace-nowrap",
    "text-warcraft-gold",
    "[text-shadow:1px_1px_0_#000,0_0_14px_rgba(255,206,99,0.45)]",
    "data-[compact=true]:text-[2.4rem]",
    "data-[conflict=true]:text-[#ff5a5a]",
    "data-[conflict=true]:[text-shadow:1px_1px_0_#000,0_0_14px_rgba(255,90,90,0.55)]",
];

const MOBILE: &[TailwindClass] = tw![
    "mobile:text-[clamp(11px,3.6vw,15px)]",
    "mobile:tracking-[0.02em]",
    "mobile:data-[compact=true]:text-[clamp(14px,4vw,18px)]",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:text-[clamp(11px,3.6vw,15px)]",
    "tablet:tracking-[0.02em]",
    "tablet:data-[compact=true]:text-[clamp(14px,4vw,18px)]",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
