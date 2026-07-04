use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "flex-1",
    "min-h-0",
    "flex",
    "flex-col",
    "gap-6",
    "pt-[2.4rem]",
    "px-[3rem]",
    "pb-[2.6rem]",
    "overflow-y-auto",
];

const MOBILE: &[TailwindClass] = tw![
    "mobile:pt-[1.25rem]",
    "mobile:px-[1rem]",
    "mobile:pb-[1.5rem]",
];

const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
