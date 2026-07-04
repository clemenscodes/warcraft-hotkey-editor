use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "flex",
    "items-center",
    "justify-center",
    "w-9",
    "h-9",
    "rounded-md",
    "border-0",
    "bg-transparent",
    "text-warcraft-text-muted",
    "cursor-pointer",
    "transition-[color,background-color,border-color]",
    "duration-150",
    "text-[2rem]",
    "leading-none",
    "hover:text-warcraft-gold",
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
