use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "block",
    "h-[2.4rem]",
    "w-auto",
    "flex-none",
    "[filter:drop-shadow(0_1px_0_color-mix(in_oklab,var(--color-warcraft-shadow)_70%,transparent))]",
];

const MOBILE: &[TailwindClass] = tw!["mobile:w-[2rem]"];
const TABLET: &[TailwindClass] = tw!["tablet:w-[2.75rem]"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
