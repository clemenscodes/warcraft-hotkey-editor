use crate::{classes, styling::TailwindClass, tw};
const BASE: &[TailwindClass] = tw![
    "group",
    "relative",
    "inline-flex",
    "flex-none",
    "m-0",
    "p-0",
    "bg-transparent",
    "border-none",
    "cursor-pointer",
    "leading-[0]",
    "disabled:cursor-default",
];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
