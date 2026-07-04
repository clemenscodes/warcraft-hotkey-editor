use crate::{classes, styling::TailwindClass, tw};
const BASE: &[TailwindClass] = tw![
    "relative",
    "flex",
    "items-center",
    "justify-center",
    "gap-[1.6rem]",
    "w-full",
    "mt-[1.4rem]",
];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
