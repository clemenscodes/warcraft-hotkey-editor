use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "px-[3rem]",
    "py-[1.4rem]",
    "border",
    "border-[#6c5a1f]",
    "rounded-[10px]",
    "text-[2rem]",
    "tracking-[0.06em]",
    "uppercase",
    "text-warcraft-gold",
    "cursor-pointer",
    "[background:linear-gradient(180deg,rgba(40,30,8,0.65)_0%,rgba(15,12,4,0.65)_100%)]",
    "[text-shadow:1px_1px_0_#000]",
    "[transition:border-color_0.15s_ease,background_0.15s_ease,box-shadow_0.15s_ease]",
    "hover:border-warcraft-gold",
    "hover:[background:linear-gradient(180deg,rgba(255,206,99,0.18)_0%,rgba(40,30,8,0.65)_100%)]",
    "hover:[box-shadow:0_0_10px_rgba(255,206,99,0.35)]",
    "active:translate-y-[1px]",
];

const MOBILE: &[TailwindClass] = tw![
    "mobile:w-full",
    "mobile:min-h-[44px]",
    "mobile:px-[24px]",
    "mobile:py-[12px]",
    "mobile:text-[16px]",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:w-full",
    "tablet:min-h-[44px]",
    "tablet:px-[24px]",
    "tablet:py-[12px]",
    "tablet:text-[16px]",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
