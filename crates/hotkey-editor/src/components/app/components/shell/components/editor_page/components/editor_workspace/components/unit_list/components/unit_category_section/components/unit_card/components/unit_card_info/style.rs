use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw!["flex", "flex-col", "gap-[0.45rem]", "min-w-0", "flex-1"];

const MOBILE: &[TailwindClass] = tw![
    "mobile:items-start",
    "mobile:text-left",
    "mobile:gap-1",
    "mobile:overflow-hidden",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:items-start",
    "tablet:text-left",
    "tablet:gap-1",
    "tablet:overflow-hidden",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
