use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "flex-[0_1_auto]",
    "min-w-0",
    "text-[inherit]",
    "text-warcraft-gold/90",
    "overflow-hidden",
    "text-ellipsis",
    "whitespace-nowrap",
    "group-data-[regen=true]:text-warcraft-gold/70",
    "group-data-[primary=true]:text-warcraft-gold",
    "group-data-[regen=true]:text-[clamp(1.3rem,0.85rem+0.32vw,1.75rem)]",
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
