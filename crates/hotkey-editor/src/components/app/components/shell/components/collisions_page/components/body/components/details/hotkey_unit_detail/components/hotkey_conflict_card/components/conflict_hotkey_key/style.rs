use crate::{classes, styling::TailwindClass, tw};
const BASE: &[TailwindClass] = tw![
    "box-border",
    "min-w-[60px]",
    "h-[60px]",
    "px-[11px]",
    "inline-flex",
    "items-center",
    "justify-center",
    "text-[34px]",
    "leading-[1]",
    "text-warcraft-gold",
    "bg-warcraft-gold/12",
    "border-2",
    "border-warcraft-gold",
    "rounded-[8px]",
    "[text-shadow:1px_1px_0_var(--color-warcraft-shadow)]",
];
const MOBILE: &[TailwindClass] = tw![
    "mobile:w-[56px]",
    "mobile:min-w-0",
    "mobile:h-[56px]",
    "mobile:p-0",
    "mobile:text-[30px]",
];
const TABLET: &[TailwindClass] = tw![
    "tablet:w-[56px]",
    "tablet:min-w-0",
    "tablet:h-[56px]",
    "tablet:p-0",
    "tablet:text-[30px]",
];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
