use crate::{classes, styling::TailwindClass, tw};

// The instruction line at the top of a position-picker dialog.
const BASE: &[TailwindClass] = tw![
    "m-0",
    "text-center",
    "max-w-[90rem]",
    "uppercase",
    "tracking-[0.1em]",
    "text-warcraft-gold/75",
    "text-[1.85rem]",
    "leading-[1.4]",
    "text-shadow-drop",
];

const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
