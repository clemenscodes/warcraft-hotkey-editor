use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "inline-flex",
    "items-center",
    "justify-center",
    "w-[1.15em]",
    "h-[1.15em]",
    "text-rose-400/90",
    "drop-shadow-[0_0_0.3em_color-mix(in_oklab,var(--color-race-orc)_35%,transparent)]",
    "[&_svg]:block",
    "[&_svg]:w-full",
    "[&_svg]:h-full",
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
