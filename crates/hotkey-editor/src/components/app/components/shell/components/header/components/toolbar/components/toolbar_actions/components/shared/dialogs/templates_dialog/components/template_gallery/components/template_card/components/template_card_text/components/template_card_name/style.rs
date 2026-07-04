use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "m-0",
    "font-friz-quadrata",
    "text-[2.75rem]",
    "uppercase",
    "tracking-[0.08em]",
    "text-inherit",
    "[text-shadow:1px_1px_0_#000]",
];

const MOBILE: &[TailwindClass] = tw![
    "mobile:text-[clamp(17px,5vw,24px)]",
    "mobile:tracking-[0.06em]",
    "mobile:text-warcraft-gold",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:text-[clamp(17px,5vw,24px)]",
    "tablet:tracking-[0.06em]",
    "tablet:text-warcraft-gold",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
