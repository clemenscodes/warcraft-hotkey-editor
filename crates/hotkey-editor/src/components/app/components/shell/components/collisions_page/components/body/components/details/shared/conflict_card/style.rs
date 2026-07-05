use crate::{classes, styling::TailwindClass, tw};
const BASE: &[TailwindClass] = tw![
    "flex",
    "flex-col",
    "items-center",
    "gap-[22px]",
    "p-[24px_16px]",
    "bg-warcraft-bg-mid/45",
    "border",
    "border-warcraft-blue-bright-deep",
    "rounded-[10px]",
    "min-w-0",
];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
