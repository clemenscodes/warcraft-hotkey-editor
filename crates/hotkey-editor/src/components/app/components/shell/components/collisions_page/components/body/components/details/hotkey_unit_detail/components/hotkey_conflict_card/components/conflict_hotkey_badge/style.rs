use crate::{classes, styling::TailwindClass, tw};
const BASE: &[TailwindClass] = tw![
    "self-start",
    "h-[72px]",
    "inline-flex",
    "items-center",
    "justify-center",
    "data-[top=true]:mb-2",
];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
