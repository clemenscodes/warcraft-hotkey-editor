use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "flex",
    "flex-col",
    "[grid-column:2/3]",
    "self-start",
    "w-full",
    "min-w-0",
    "min-h-0",
    "max-h-full",
    "p-[clamp(0.9rem,1.2vh,1.25rem)_clamp(1rem,1vw,1.5rem)]",
    "pb-[clamp(1rem,1.5vh,1.5rem)]",
    "gap-[clamp(0.95rem,1.6vh,1.5rem)]",
    "border",
    "border-warcraft-blue-deep",
    "rounded-[12px]",
    "bg-panel-dark-diag-70",
    "shadow-bevel-hl-3",
    "overflow-y-auto",
    "overflow-x-clip",
    "[overscroll-behavior:contain]",
    "[scrollbar-width:thin]",
    "[scrollbar-color:color-mix(in_oklab,var(--color-warcraft-gold)_35%,transparent)_transparent]",
    "[&::-webkit-scrollbar]:w-[5px]",
    "[&::-webkit-scrollbar-track]:bg-transparent",
    "[&::-webkit-scrollbar-thumb]:bg-warcraft-gold/35",
    "[&::-webkit-scrollbar-thumb]:rounded-[3px]",
    "[&::-webkit-scrollbar-thumb:hover]:bg-warcraft-gold/55",
];
const MOBILE: &[TailwindClass] = tw![
    "mobile:[grid-column:auto]",
    "mobile:self-auto",
    "mobile:max-h-none",
    "mobile:w-full",
    "mobile:box-border",
    "mobile:p-[1.25rem]",
    "mobile:rounded-[8px]",
    "mobile:overflow-visible",
];
const TABLET: &[TailwindClass] = tw![
    "tablet:[grid-column:auto]",
    "tablet:self-stretch",
    "tablet:max-h-none",
    "tablet:w-full",
    "tablet:box-border",
    "tablet:p-[1.25rem]",
    "tablet:rounded-[8px]",
    "tablet:overflow-visible",
];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
