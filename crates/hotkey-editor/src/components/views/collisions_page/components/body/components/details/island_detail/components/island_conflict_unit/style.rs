use crate::{classes, styling::TailwindClass, tw};
const BASE: &[TailwindClass] = tw![
    "m-0",
    "p-0",
    "bg-transparent",
    "border-none",
    "cursor-pointer",
    "flex",
    "flex-col",
    "items-center",
    "gap-[7px]",
    "max-w-full",
];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
