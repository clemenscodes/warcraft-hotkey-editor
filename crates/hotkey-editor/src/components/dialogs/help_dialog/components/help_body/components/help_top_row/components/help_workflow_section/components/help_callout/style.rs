use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "px-[2.2rem]",
    "py-[1.8rem]",
    "border",
    "border-warcraft-gold/45",
    "rounded-xl",
    "bg-[linear-gradient(180deg,rgba(40,30,8,0.45)_0%,rgba(15,12,4,0.35)_100%)]",
    "shadow-[inset_0_0_0_1px_rgba(255,206,99,0.08),0_0_18px_rgba(255,206,99,0.12)]",
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
