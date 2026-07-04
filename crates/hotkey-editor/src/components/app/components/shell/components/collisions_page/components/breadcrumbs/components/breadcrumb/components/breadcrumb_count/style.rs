use crate::{classes, styling::TailwindClass, tw};
const BASE: &[TailwindClass] = tw![
    "font-mono",
    "text-[1.6rem]",
    "opacity-80",
    "before:content-['(']",
    "after:content-[')']",
    "group-data-[active=true]:opacity-100",
];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
