use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "flex",
    "items-center",
    "justify-between",
    "gap-8",
    "px-8",
    "py-5",
    "[border-top:1px_solid_rgba(255,206,99,0.14)]",
    "last:[border-bottom:1px_solid_rgba(255,206,99,0.14)]",
];

const MOBILE: &[TailwindClass] = tw![
    "mobile:gap-[0.75rem]",
    "mobile:px-[0.5rem]",
    "mobile:py-[0.7rem]",
    "mobile:[touch-action:pan-y]",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:gap-[0.75rem]",
    "tablet:px-[0.5rem]",
    "tablet:py-[0.7rem]",
    "tablet:[touch-action:pan-y]",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
