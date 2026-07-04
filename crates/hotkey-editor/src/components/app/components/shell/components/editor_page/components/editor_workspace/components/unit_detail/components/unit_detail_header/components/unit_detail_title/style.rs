use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "flex",
    "flex-col",
    "min-w-0",
    "gap-[0.45rem]",
    "overflow-x-clip",
];
const MOBILE: &[TailwindClass] = tw![
    "mobile:flex-1",
    "mobile:items-start",
    "mobile:gap-[3px]",
    "mobile:text-left",
    "mobile:overflow-visible",
];
const TABLET: &[TailwindClass] = tw![
    "tablet:flex-1",
    "tablet:items-start",
    "tablet:gap-[3px]",
    "tablet:text-left",
    "tablet:overflow-visible",
];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
