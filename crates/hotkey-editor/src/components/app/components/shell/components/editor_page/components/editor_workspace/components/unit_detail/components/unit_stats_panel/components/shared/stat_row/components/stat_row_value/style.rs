use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "flex-[1_1_auto]",
    "min-w-0",
    "text-right",
    "text-warcraft-text-primary",
    "font-medium",
    "[font-variant-numeric:tabular-nums]",
    "group-data-[variant=hp]:not-data-[zero=true]:text-warcraft-success",
    "group-data-[variant=hp]:font-semibold",
    "group-data-[variant=hp]:text-[clamp(1.7rem,1.05rem+0.48vw,2.2rem)]",
    "group-data-[variant=mana]:not-data-[zero=true]:text-race-human",
    "group-data-[variant=mana]:font-semibold",
    "group-data-[variant=mana]:text-[clamp(1.7rem,1.05rem+0.48vw,2.2rem)]",
    "data-[zero=true]:text-warcraft-text-faint",
    "data-[zero=true]:font-normal",
];
const MOBILE: &[TailwindClass] = tw![
    "mobile:group-data-[variant=hp]:text-[2.6rem]",
    "mobile:group-data-[variant=mana]:text-[2.6rem]",
];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
