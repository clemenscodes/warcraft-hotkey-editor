use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "flex-[0_0_auto]",
    "ml-auto",
    "text-[1.2rem]",
    "italic",
    "text-warcraft-text-secondary/55",
    "[text-shadow:1px_1px_0_var(--color-warcraft-shadow)]",
];
const MOBILE: &[TailwindClass] = tw!["mobile:text-[1.55rem]"];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
