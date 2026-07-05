use crate::{classes, styling::TailwindClass, tw};

// The bold light-blue caption naming the off-state / upgraded form.
const BASE: &[TailwindClass] = tw![
    "m-0",
    "font-semibold",
    "text-warcraft-text-secondary",
    "text-[1.45rem]"
];
const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
