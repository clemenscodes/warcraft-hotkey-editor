use crate::{classes, styling::TailwindClass, tw};

// The crosshair button that opens the position picker for an off-state / upgraded-
// form button. Blue-themed square matching the alt-state block; the crosshair SVG
// fills it. Shared by the alt-state and upgrade sections.
const BASE: &[TailwindClass] = tw![
    "appearance-none",
    "w-20",
    "h-20",
    "p-[0.3rem]",
    "inline-flex",
    "items-center",
    "justify-center",
    "bg-[rgba(110,195,255,0.08)]",
    "border-2",
    "border-[#6ec3ff]",
    "text-[#a9d8ff]",
    "rounded-[4px]",
    "cursor-pointer",
    "transition-[background,border-color,color]",
    "duration-[0.12s]",
    "hover:bg-[rgba(110,195,255,0.22)]",
    "hover:border-[#6ec3ff]",
    "hover:text-[#d6ecff]",
    "focus-visible:[outline:2px_solid_#6ec3ff]",
    "focus-visible:[outline-offset:2px]",
    "[&>svg]:block",
    "[&>svg]:w-full",
    "[&>svg]:h-full",
];

const MOBILE: &[TailwindClass] = tw!["mobile:w-[4.6rem]", "mobile:h-[4.6rem]"];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
