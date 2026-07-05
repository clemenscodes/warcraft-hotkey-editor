use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "flex-[0_0_auto]",
    "text-warcraft-success",
    "text-[clamp(1.3rem,0.85rem+0.32vw,1.75rem)]",
    "font-normal",
    "[font-variant-numeric:tabular-nums]",
    "text-shadow-drop",
    "group-data-[regen=true]:ml-auto",
    "group-data-[regen=true]:text-right",
    "group-data-[regen=true]:group-data-[variant=mana]:not-data-[zero=true]:text-race-human",
    "data-[zero=true]:text-warcraft-text-faint",
    "data-[zero=true]:font-normal",
];
const MOBILE: &[TailwindClass] = tw!["mobile:text-[1.7rem]"];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
