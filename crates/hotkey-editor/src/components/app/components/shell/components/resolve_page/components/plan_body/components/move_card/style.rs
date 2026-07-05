use crate::{classes, styling::TailwindClass, tw};
const BASE: &[TailwindClass] = tw![
    "flex",
    "flex-col",
    "gap-[1.2rem]",
    "p-[1.4rem_1.5rem]",
    "box-border",
    "bg-warcraft-bg-mid/45",
    "border",
    "border-warcraft-blue-bright-deep",
    "rounded-[10px]",
    "data-[stuck=true]:border-race-orc/50",
];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
