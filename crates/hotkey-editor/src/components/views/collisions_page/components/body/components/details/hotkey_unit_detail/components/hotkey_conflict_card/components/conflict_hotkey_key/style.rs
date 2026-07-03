use crate::classes;
const BASE: &[&str] = &[
    "box-border",
    "min-w-[60px]",
    "h-[60px]",
    "px-[11px]",
    "inline-flex",
    "items-center",
    "justify-center",
    "font-friz-quadrata",
    "text-[34px]",
    "leading-[1]",
    "text-warcraft-gold",
    "bg-[rgba(255,206,99,0.12)]",
    "border-2",
    "border-warcraft-gold",
    "rounded-[8px]",
    "[text-shadow:1px_1px_0_#000]",
];
const MOBILE: &[&str] = &[
    "mobile:w-[56px]",
    "mobile:min-w-0",
    "mobile:h-[56px]",
    "mobile:p-0",
    "mobile:text-[30px]",
];
const TABLET: &[&str] = &[
    "tablet:w-[56px]",
    "tablet:min-w-0",
    "tablet:h-[56px]",
    "tablet:p-0",
    "tablet:text-[30px]",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
