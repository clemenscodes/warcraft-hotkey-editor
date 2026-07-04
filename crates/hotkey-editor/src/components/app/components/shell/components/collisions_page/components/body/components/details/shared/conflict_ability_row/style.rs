use crate::{classes, styling::TailwindClass, tw};
const BASE: &[TailwindClass] = tw![
    "grid",
    "grid-cols-[1fr_auto_1fr]",
    "items-start",
    "justify-items-center",
    "gap-[12px]",
    "w-full",
    "data-[multi=true]:grid-cols-none",
    "data-[multi=true]:flex",
    "data-[multi=true]:flex-wrap",
    "data-[multi=true]:justify-center",
];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
