use crate::classes;

const BASE: &[&str] = &[
    "inline-flex",
    "items-center",
    "gap-4",
    "h-24",
    "px-8",
    "border",
    "border-warcraft-gold",
    "rounded-[12px]",
    "text-warcraft-gold",
    "font-mono",
    "text-[2rem]",
    "tracking-[0.14em]",
    "font-medium",
    "cursor-pointer",
    "[background:linear-gradient(135deg,rgba(40,30,8,0.85)_0%,rgba(15,12,4,0.85)_100%)]",
    "[box-shadow:0_0_22px_rgba(255,206,99,0.22)]",
    "[transition:background_0.12s_ease,box-shadow_0.12s_ease,transform_0.12s_ease]",
    "focus:outline-none",
    "focus-visible:border-white",
    "focus-visible:text-white",
    "focus-visible:[box-shadow:0_0_0_3px_#fff,0_0_18px_rgba(255,255,255,0.55)]",
    "hover:[background:linear-gradient(135deg,rgba(255,206,99,0.22)_0%,rgba(60,45,14,0.95)_100%)]",
    "hover:[box-shadow:0_0_26px_rgba(255,206,99,0.55),inset_0_0_14px_rgba(255,206,99,0.15)]",
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
