use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "px-8",
    "py-4",
    "rounded-md",
    "border",
    "border-warcraft-gold/35",
    "bg-warcraft-bg-base/85",
    "text-[2rem]",
    "text-warcraft-gold",
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
