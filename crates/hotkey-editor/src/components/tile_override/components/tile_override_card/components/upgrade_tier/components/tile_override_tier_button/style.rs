use crate::classes;

// A prev/next arrow button in the tier-cycling footer. Small bronze square that
// golds on hover; the injected arrow SVG is centered and fixed-size. Class
// `.tile-override-tier-button` is load-bearing (keyboard navigation).
const BASE: &[&str] = &[
    "w-[2.4rem]",
    "h-[2.4rem]",
    "p-0",
    "flex",
    "items-center",
    "justify-center",
    "bg-[rgba(40,30,8,0.55)]",
    "border",
    "border-[#6c5a1f]",
    "rounded-[0.25rem]",
    "cursor-pointer",
    "transition-[border-color,background]",
    "duration-[0.12s]",
    "hover:border-warcraft-gold",
    "hover:bg-[rgba(255,206,99,0.12)]",
    "[&>span]:block",
    "[&_svg]:w-[1.7rem]",
    "[&_svg]:h-[1.7rem]",
];

const MOBILE: &[&str] = &[
    "mobile:w-[34px]",
    "mobile:h-[34px]",
    "mobile:min-w-[34px]",
    "mobile:min-h-[34px]",
    "mobile:[&_svg]:w-[22px]",
    "mobile:[&_svg]:h-[22px]",
];

const TABLET: &[&str] = &[
    "tablet:w-[34px]",
    "tablet:h-[34px]",
    "tablet:min-w-[34px]",
    "tablet:min-h-[34px]",
    "tablet:[&_svg]:w-[22px]",
    "tablet:[&_svg]:h-[22px]",
];

const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
