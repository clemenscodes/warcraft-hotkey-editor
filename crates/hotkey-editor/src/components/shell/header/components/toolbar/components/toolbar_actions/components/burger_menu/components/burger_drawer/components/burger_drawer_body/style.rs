use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "flex-1",
    "flex",
    "flex-col",
    "gap-5",
    "py-6",
    "px-5",
    "overflow-y-auto",
];

const MOBILE: &[TailwindClass] = tw![
    "mobile:gap-4",
    "mobile:pt-5",
    "mobile:px-4",
    "mobile:pb-[max(1.25rem,env(safe-area-inset-bottom))]",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:gap-4",
    "tablet:pt-5",
    "tablet:px-4",
    "tablet:pb-[max(1.25rem,env(safe-area-inset-bottom))]",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
