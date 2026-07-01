use crate::classes;

// The single source of truth for how a toolbar action button looks: a square
// bronze-bordered gradient tile, secondary-text by default, gold on hover and a
// white ring on keyboard focus.
const BASE: &[&str] = &[
    "inline-flex",
    "items-center",
    "justify-center",
    "w-20",
    "h-20",
    "p-0",
    "border",
    "border-[#6c5a1f]",
    "rounded-[12px]",
    "text-warcraft-text-secondary",
    "cursor-pointer",
    "[background:linear-gradient(180deg,rgba(40,30,8,0.55)_0%,rgba(15,12,4,0.55)_100%)]",
    "[transition:border-color_0.15s_ease,color_0.15s_ease,background_0.15s_ease,box-shadow_0.15s_ease]",
    "focus:outline-none",
    "focus-visible:border-white",
    "focus-visible:text-white",
    "focus-visible:[box-shadow:0_0_0_3px_#fff,0_0_16px_rgba(255,255,255,0.55)]",
    "hover:border-warcraft-gold",
    "hover:text-warcraft-gold",
    "hover:[background:linear-gradient(180deg,rgba(255,206,99,0.18)_0%,rgba(40,30,8,0.55)_100%)]",
    "hover:[box-shadow:0_0_12px_rgba(255,206,99,0.3)]",
];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
