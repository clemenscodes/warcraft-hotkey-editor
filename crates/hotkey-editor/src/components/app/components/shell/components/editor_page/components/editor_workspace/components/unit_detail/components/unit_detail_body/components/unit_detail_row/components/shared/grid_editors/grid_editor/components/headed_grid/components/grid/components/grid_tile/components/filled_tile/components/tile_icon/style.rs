use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "w-full",
    "h-full",
    "object-cover",
    "text-[0px]",
    "leading-[0]",
    "text-transparent",
    "[background:radial-gradient(circle_at_center,color-mix(in_oklab,var(--color-warcraft-gold)_8%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-gold)_0%,transparent)_65%)]",
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
