use crate::{classes, styling::TailwindClass, tw};
const BASE: &[TailwindClass] = tw![
    "m-0",
    "p-0",
    "bg-transparent",
    "border-none",
    "cursor-pointer",
    "text-[14px]",
    "text-warcraft-text-muted",
    "underline",
    "underline-offset-2",
    "hover:text-warcraft-gold",
];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
