use crate::classes;

const BASE: &[&str] = &[
    "absolute",
    "right-[1rem]",
    "top-1/2",
    "-translate-y-1/2",
    "w-[2.5rem]",
    "h-[2.5rem]",
    "text-[1.5rem]",
    "font-friz-quadrata",
    "flex",
    "items-center",
    "justify-center",
    "bg-transparent",
    "border-0",
    "cursor-pointer",
    "text-warcraft-text-secondary",
    "[text-shadow:1px_1px_0_#000]",
    "transition-[color,text-shadow]",
    "duration-150",
    "hover:text-warcraft-gold",
    "hover:[text-shadow:1px_1px_0_#000,0_0_12px_rgba(255,206,99,0.55)]",
    "focus:outline-none",
    "kb-focus:text-white",
    "kb-focus:[text-shadow:1px_1px_0_#000,0_0_16px_rgba(255,255,255,0.7)]",
];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
