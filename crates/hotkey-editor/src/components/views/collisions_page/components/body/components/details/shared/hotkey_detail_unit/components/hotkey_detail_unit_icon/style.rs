use crate::{classes, styling::TailwindClass, tw};
const BASE: &[TailwindClass] = tw![
    "h-[80px]",
    "w-[80px]",
    "border",
    "border-warcraft-blue",
    "rounded-[4px]",
    "object-cover",
];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
