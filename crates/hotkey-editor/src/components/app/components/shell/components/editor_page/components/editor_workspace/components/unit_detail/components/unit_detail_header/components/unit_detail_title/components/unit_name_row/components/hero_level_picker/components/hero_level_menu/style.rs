use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "absolute",
    "top-[calc(100%+4px)]",
    "left-0",
    "z-[200]",
    "min-w-full",
    "flex",
    "flex-col",
    "gap-[2px]",
    "p-[4px]",
    "[background:linear-gradient(170deg,rgba(12,25,50,0.98)_0%,rgba(6,12,28,0.98)_100%)]",
    "border",
    "border-[rgba(255,206,99,0.45)]",
    "rounded-[8px]",
    "[box-shadow:0_8px_24px_rgba(0,0,0,0.65),0_0_12px_rgba(255,206,99,0.1)]",
];
const MOBILE: &[TailwindClass] = tw![
    "mobile:w-full",
    "mobile:max-h-[min(320px,65dvh)]",
    "mobile:overflow-y-auto",
    "mobile:[overscroll-behavior:contain]",
    "mobile:[scrollbar-width:none]",
    "mobile:[&::-webkit-scrollbar]:hidden",
];
const TABLET: &[TailwindClass] = tw![
    "tablet:w-full",
    "tablet:max-h-[min(320px,65dvh)]",
    "tablet:overflow-y-auto",
    "tablet:[overscroll-behavior:contain]",
    "tablet:[scrollbar-width:none]",
    "tablet:[&::-webkit-scrollbar]:hidden",
];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
