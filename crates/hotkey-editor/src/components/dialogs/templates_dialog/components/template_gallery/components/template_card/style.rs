use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "flex",
    "flex-col",
    "gap-9",
    "py-10",
    "px-11",
    "border",
    "border-[#6c5a1f]",
    "rounded-[14px]",
    "text-left",
    "text-[#c0c8da]",
    "bg-[linear-gradient(180deg,rgba(40,30,8,0.55)_0%,rgba(15,12,4,0.55)_100%)]",
    "cursor-pointer",
    "transition-[border-color,color,background,box-shadow]",
    "duration-150",
    "ease-[ease]",
    "hover:border-warcraft-gold",
    "hover:text-warcraft-gold",
    "hover:bg-[linear-gradient(180deg,rgba(255,206,99,0.18)_0%,rgba(40,30,8,0.55)_100%)]",
    "hover:shadow-[0_0_14px_rgba(255,206,99,0.35)]",
    "focus:outline-none",
    "kb-focus:outline-none",
    "kb-focus:border-[#fff]",
    "kb-focus:text-[#fff]",
    "kb-focus:shadow-[0_0_0_3px_#fff,0_0_16px_rgba(255,255,255,0.55)]",
];

const MOBILE: &[TailwindClass] = tw![
    "mobile:gap-[10px]",
    "mobile:py-[12px]",
    "mobile:px-[14px]",
    "mobile:rounded-[10px]",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:gap-[10px]",
    "tablet:py-[12px]",
    "tablet:px-[14px]",
    "tablet:rounded-[10px]",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
