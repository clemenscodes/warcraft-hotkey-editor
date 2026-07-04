use crate::{classes, styling::TailwindClass, tw};
const BASE: &[TailwindClass] = tw![
    "text-[22px]",
    "leading-[1.3]",
    "text-warcraft-text-primary",
    "text-center",
    "max-w-full",
    "whitespace-nowrap",
];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
