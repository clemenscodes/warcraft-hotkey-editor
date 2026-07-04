use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw!["grid", "grid-cols-[repeat(10,11rem)]", "gap-[0.8rem]"];

const MOBILE: &[TailwindClass] = tw![
    "mobile:grid-cols-[repeat(5,minmax(0,1fr))]",
    "mobile:auto-rows-[minmax(72px,auto)]",
    "mobile:gap-[0.4rem]",
    "mobile:w-full",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:grid-cols-[repeat(5,minmax(0,1fr))]",
    "tablet:auto-rows-[minmax(72px,auto)]",
    "tablet:gap-[0.4rem]",
    "tablet:w-full",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
