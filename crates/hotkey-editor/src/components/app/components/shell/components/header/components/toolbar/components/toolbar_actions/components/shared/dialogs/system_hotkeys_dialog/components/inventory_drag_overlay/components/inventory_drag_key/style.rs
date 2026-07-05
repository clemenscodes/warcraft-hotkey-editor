use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "uppercase",
    "tracking-[0.04em]",
    "text-[3.4rem]",
    "leading-none",
    "whitespace-nowrap",
    "text-warcraft-gold",
    "[text-shadow:1px_1px_0_var(--color-warcraft-shadow),0_0_14px_color-mix(in_oklab,var(--color-warcraft-gold)_45%,transparent)]",
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
