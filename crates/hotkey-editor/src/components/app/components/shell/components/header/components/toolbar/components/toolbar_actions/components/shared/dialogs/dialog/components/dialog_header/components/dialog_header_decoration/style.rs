use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "block",
    "h-[2.4rem]",
    "w-auto",
    "flex-none",
    "[filter:drop-shadow(0_1px_0_rgba(0,0,0,0.7))]",
];

const MOBILE: &[TailwindClass] = tw!["mobile:w-[2rem]"];
const TABLET: &[TailwindClass] = tw!["tablet:w-[2.75rem]"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
