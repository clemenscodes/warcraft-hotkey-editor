use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "m-0",
    "text-[1.8rem]/[1.3]",
    "text-warcraft-gold",
    "[text-shadow:1px_1px_0_#000]",
];

const MOBILE: &[TailwindClass] = tw!["mobile:text-[1.5rem]/[1.3]"];
const TABLET: &[TailwindClass] = tw!["tablet:text-[1.5rem]/[1.3]"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
