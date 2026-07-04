use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "inline-flex",
    "items-center",
    "justify-center",
    "shrink-0",
    "w-12",
    "h-12",
    "border",
    "border-warcraft-gold-soft",
    "rounded-lg",
    "bg-[linear-gradient(180deg,rgba(40,30,8,0.55)_0%,rgba(15,12,4,0.55)_100%)]",
    "text-warcraft-gold",
    "[&_svg]:w-8",
    "[&_svg]:h-8",
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
