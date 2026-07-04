use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw!["flex", "flex-row", "items-end", "gap-[0.8rem]"];

const MOBILE: &[TailwindClass] = tw![
    "mobile:flex-col",
    "mobile:items-center",
    "mobile:gap-[0.4rem]",
];

const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
