use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "w-full",
    "m-0",
    "px-6",
    "py-5",
    "rounded-md",
    "border",
    "border-[rgba(255,180,0,0.45)]",
    "bg-[rgba(60,40,0,0.45)]",
    "text-center",
    "font-friz-quadrata",
    "uppercase",
    "tracking-[0.08em]",
    "text-[1.75rem]/[1.625]",
    "text-warcraft-gold/85",
    "[text-shadow:1px_1px_0_#000]",
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
