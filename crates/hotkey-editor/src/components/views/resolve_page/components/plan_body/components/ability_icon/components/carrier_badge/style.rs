use crate::classes;
const BASE: &[&str] = &[
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
    "bg-[#122742]",
    "border",
    "border-[#3a5277]",
    "text-[#c2c8d2]",
    "font-mono",
    "text-[1.05rem]",
    "leading-[1]",
    "[text-shadow:1px_1px_0_#000]",
    "data-[win=true]:border-warcraft-gold",
    "data-[win=true]:text-warcraft-gold",
];
const MOBILE: &[&str] = &[
    "mobile:min-w-[15px]",
    "mobile:h-[15px]",
    "mobile:text-[0.85rem]",
];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
