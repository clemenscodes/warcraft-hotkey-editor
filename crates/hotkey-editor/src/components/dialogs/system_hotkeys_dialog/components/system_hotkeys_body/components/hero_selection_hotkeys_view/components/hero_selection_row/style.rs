use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw!["grid", "grid-cols-[repeat(3,26rem)]", "gap-[1.5rem]"];

const MOBILE: &[TailwindClass] = tw![
    "mobile:grid-cols-[repeat(3,minmax(0,1fr))]",
    "mobile:gap-[0.5rem]",
    "mobile:w-full",
    "mobile:max-w-[30rem]",
    "mobile:mx-auto",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:grid-cols-[repeat(3,minmax(0,1fr))]",
    "tablet:gap-[0.5rem]",
    "tablet:w-full",
    "tablet:max-w-[30rem]",
    "tablet:mx-auto",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
