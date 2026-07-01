use crate::classes;

const BASE: &[&str] = &[
    "inline-flex",
    "items-center",
    "justify-center",
    "min-h-[3rem]",
    "px-[1.8rem]",
    "py-[0.7rem]",
    "border",
    "border-warcraft-gold",
    "rounded-[10px]",
    "text-warcraft-gold",
    "font-friz-quadrata",
    "text-[1.4rem]",
    "tracking-[0.08em]",
    "uppercase",
    "cursor-pointer",
    "bg-[linear-gradient(135deg,rgba(40,30,8,0.85)_0%,rgba(15,12,4,0.85)_100%)]",
    "shadow-[0_0_22px_rgba(255,206,99,0.22)]",
    "transition-[background,box-shadow]",
    "duration-[120ms]",
    "focus:outline-none",
    "focus-visible:border-white",
    "focus-visible:text-white",
    "focus-visible:shadow-[0_0_0_3px_#fff,0_0_18px_rgba(255,255,255,0.55)]",
    "hover:bg-[linear-gradient(135deg,rgba(255,206,99,0.22)_0%,rgba(60,45,14,0.95)_100%)]",
    "hover:shadow-[0_0_26px_rgba(255,206,99,0.55)]",
];

const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
