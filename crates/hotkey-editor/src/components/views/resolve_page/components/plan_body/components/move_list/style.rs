use crate::{classes, styling::TailwindClass, tw};
const BASE: &[TailwindClass] = tw![
    "grid",
    "grid-cols-[repeat(auto-fill,minmax(min(760px,100%),1fr))]",
    "gap-4",
    "content-start",
];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
