use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "fixed",
    "inset-0",
    "z-[70]",
    "bg-[rgba(0,0,0,0.65)]",
    "cursor-pointer",
    "border-none",
    "p-0",
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
