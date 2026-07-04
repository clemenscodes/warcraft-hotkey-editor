use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "flex",
    "flex-col",
    "items-center",
    "justify-center",
    "gap-[4rem]",
];

const MOBILE: &[TailwindClass] = tw!["mobile:justify-start", "mobile:gap-[20px]"];
const TABLET: &[TailwindClass] = tw!["tablet:justify-start", "tablet:gap-[20px]"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
