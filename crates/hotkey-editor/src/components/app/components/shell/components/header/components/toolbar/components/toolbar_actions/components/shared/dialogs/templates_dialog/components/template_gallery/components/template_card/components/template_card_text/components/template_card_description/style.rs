use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw!["m-0", "text-[1.6rem]/[1.45]", "opacity-80"];

const MOBILE: &[TailwindClass] = tw![
    "mobile:text-[13px]/[1.35]",
    "mobile:text-[#c0c8da]",
    "mobile:opacity-90",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:text-[13px]/[1.35]",
    "tablet:text-[#c0c8da]",
    "tablet:opacity-90",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
