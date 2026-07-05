use crate::{classes, styling::TailwindClass, tw};
const BASE: &[TailwindClass] = tw![
    "absolute",
    "top-[-6px]",
    "right-[-6px]",
    "min-w-[19px]",
    "h-[19px]",
    "px-[4px]",
    "box-border",
    "inline-flex",
    "items-center",
    "justify-center",
    "rounded-[10px]",
    "bg-warcraft-bg-panel",
    "border",
    "border-warcraft-blue",
    "text-warcraft-text-secondary",
    "text-[1.05rem]",
    "leading-[1]",
    "text-shadow-drop",
    "data-[win=true]:border-warcraft-gold",
    "data-[win=true]:text-warcraft-gold",
];
const MOBILE: &[TailwindClass] = tw![
    "mobile:min-w-[15px]",
    "mobile:h-[15px]",
    "mobile:text-[0.85rem]",
];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
