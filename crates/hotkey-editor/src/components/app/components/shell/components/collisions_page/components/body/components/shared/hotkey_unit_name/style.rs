use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "font-friz-quadrata",
    "text-[1.5rem]",
    "text-warcraft-text-primary",
    "whitespace-normal",
    "leading-[1.15]",
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
