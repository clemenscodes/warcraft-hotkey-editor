use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "inline-flex",
    "items-center",
    "justify-center",
    "w-9",
    "h-9",
    "p-0",
    "bg-transparent",
    "border",
    "border-[rgba(255,206,99,0.3)]",
    "rounded-[8px]",
    "text-[rgba(255,206,99,0.7)]",
    "text-[0.9rem]",
    "cursor-pointer",
    "[transition:border-color_0.15s_ease,color_0.15s_ease,background_0.15s_ease]",
    "hover:border-warcraft-gold",
    "hover:text-warcraft-gold",
    "hover:bg-[rgba(255,206,99,0.08)]",
    "focus:outline-none",
    "focus-visible:border-white",
    "focus-visible:text-white",
    "focus-visible:[box-shadow:0_0_0_2px_#fff]",
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
