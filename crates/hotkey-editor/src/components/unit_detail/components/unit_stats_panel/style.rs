use crate::classes;

const BASE: &[&str] = &[
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
const MOBILE: &[&str] = &[
    "mobile:grid-cols-[minmax(0,1fr)]",
    "mobile:[grid-template-areas:'vitality'_'attributes'_'combat'_'defense']",
    "mobile:gap-5",
    "mobile:p-5",
];
const TABLET: &[&str] = &["tablet:gap-y-[1.75rem]", "tablet:p-[1.5rem_1.75rem]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
