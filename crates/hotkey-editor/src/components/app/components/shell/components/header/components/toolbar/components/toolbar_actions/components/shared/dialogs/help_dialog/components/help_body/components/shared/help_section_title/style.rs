use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "m-0",
    "uppercase",
    "tracking-widest",
    "text-[2.2rem]/[1.2]",
    "text-warcraft-gold",
    "[text-shadow:1px_1px_0_#000]",
];

const MOBILE: &[TailwindClass] = tw!["mobile:text-[1.9rem]/[1.2]", "mobile:text-center"];
const TABLET: &[TailwindClass] = tw!["tablet:text-[1.9rem]/[1.2]", "tablet:text-center"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
