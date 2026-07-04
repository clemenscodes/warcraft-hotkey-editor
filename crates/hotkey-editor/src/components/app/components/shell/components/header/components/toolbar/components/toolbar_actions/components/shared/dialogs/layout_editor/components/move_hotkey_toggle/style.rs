use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "flex",
    "items-center",
    "gap-[0.8rem]",
    "font-friz-quadrata",
    "uppercase",
    "tracking-[0.06em]",
    "text-[1.9rem]",
    "text-warcraft-gold",
    "cursor-pointer",
    "[text-shadow:1px_1px_0_#000]",
];

const MOBILE: &[TailwindClass] = tw!["mobile:gap-[8px]", "mobile:text-[15px]"];
const TABLET: &[TailwindClass] = tw!["tablet:gap-[8px]", "tablet:text-[15px]"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
