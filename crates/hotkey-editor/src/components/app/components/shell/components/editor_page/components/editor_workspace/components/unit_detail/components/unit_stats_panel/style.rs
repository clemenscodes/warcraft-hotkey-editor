use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "grid",
    "grid-cols-[repeat(2,minmax(0,1fr))]",
    "[grid-template-areas:'vitality_attributes'_'combat_defense']",
    "items-stretch",
    "gap-x-[2rem]",
    "gap-y-[2.5rem]",
    "mt-[2.5rem]",
    "mb-[0.75rem]",
    "p-[1.4rem_1.75rem]",
    "bg-[rgba(8,18,35,0.55)]",
    "border",
    "border-[#1f3d63]",
    "rounded-[8px]",
];
const MOBILE: &[TailwindClass] = tw![
    "mobile:grid-cols-[minmax(0,1fr)]",
    "mobile:[grid-template-areas:'vitality'_'attributes'_'combat'_'defense']",
    "mobile:gap-5",
    "mobile:p-5",
];
const TABLET: &[TailwindClass] = tw!["tablet:gap-y-[1.75rem]", "tablet:p-[1.5rem_1.75rem]"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
