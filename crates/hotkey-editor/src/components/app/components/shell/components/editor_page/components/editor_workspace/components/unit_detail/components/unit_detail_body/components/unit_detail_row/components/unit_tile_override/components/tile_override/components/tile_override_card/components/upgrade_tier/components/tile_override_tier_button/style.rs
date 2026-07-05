use crate::{classes, styling::TailwindClass, tw};

// A prev/next arrow button in the tier-cycling footer. Small bronze square that
// golds on hover; the injected arrow SVG is centered and fixed-size. Class
// `.tile-override-tier-button` is load-bearing (keyboard navigation).
const BASE: &[TailwindClass] = tw![
    "w-[2.4rem]",
    "h-[2.4rem]",
    "p-0",
    "flex",
    "items-center",
    "justify-center",
    "bg-warcraft-gold-dark/55",
    "border",
    "border-warcraft-gold-border",
    "rounded-[0.25rem]",
    "cursor-pointer",
    "transition-[border-color,background]",
    "duration-[0.12s]",
    "hover:border-warcraft-gold",
    "hover:bg-warcraft-gold/12",
    "[&>span]:block",
    "[&_svg]:w-[1.7rem]",
    "[&_svg]:h-[1.7rem]",
];

const MOBILE: &[TailwindClass] = tw![
    "mobile:w-[34px]",
    "mobile:h-[34px]",
    "mobile:min-w-[34px]",
    "mobile:min-h-[34px]",
    "mobile:[&_svg]:w-[22px]",
    "mobile:[&_svg]:h-[22px]",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:w-[34px]",
    "tablet:h-[34px]",
    "tablet:min-w-[34px]",
    "tablet:min-h-[34px]",
    "tablet:[&_svg]:w-[22px]",
    "tablet:[&_svg]:h-[22px]",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
