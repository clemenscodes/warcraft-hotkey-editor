use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "grid",
    "grid-cols-[repeat(2,minmax(0,1fr))]",
    "gap-x-[0.5rem]",
    "gap-y-[0.25rem]",
    "mt-auto",
    "pt-[0.75rem]",
    "border-t",
    "border-t-warcraft-gold/15",
];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
