use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "m-0",
    "max-w-[90rem]",
    "text-center",
    "font-friz-quadrata",
    "uppercase",
    "tracking-[0.1em]",
    "text-[2rem]",
    "leading-snug",
    "text-warcraft-gold/75",
    "[text-shadow:1px_1px_0_#000]",
];

const MOBILE: &[TailwindClass] = tw![
    "mobile:max-w-full",
    "mobile:px-[0.25rem]",
    "mobile:text-[clamp(11px,3vw,14px)]",
    "mobile:tracking-[0.04em]",
    "mobile:leading-[1.35]",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:max-w-full",
    "tablet:px-[0.25rem]",
    "tablet:text-[clamp(11px,3vw,14px)]",
    "tablet:tracking-[0.04em]",
    "tablet:leading-[1.35]",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
