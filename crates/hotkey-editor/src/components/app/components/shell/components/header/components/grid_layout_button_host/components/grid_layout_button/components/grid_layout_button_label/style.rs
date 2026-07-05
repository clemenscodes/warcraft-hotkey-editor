use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "font-normal",
    "uppercase",
    "whitespace-nowrap",
    "tracking-[0.12em]",
    "text-shadow-drop-60",
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
