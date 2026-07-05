use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "w-full",
    "m-0",
    "px-6",
    "py-5",
    "rounded-md",
    "border",
    "border-warcraft-gold/45",
    "bg-warcraft-gold-dark/45",
    "text-center",
    "uppercase",
    "tracking-[0.08em]",
    "text-[1.75rem]/[1.625]",
    "text-warcraft-gold/85",
    "text-shadow-drop",
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
