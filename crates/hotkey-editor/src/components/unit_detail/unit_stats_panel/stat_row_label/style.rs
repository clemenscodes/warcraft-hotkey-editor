use crate::classes;

const BASE: &[&str] = &[
    "flex-[0_1_auto]",
    "min-w-0",
    "font-friz-quadrata",
    "text-[inherit]",
    "text-warcraft-gold/90",
    "overflow-hidden",
    "text-ellipsis",
    "whitespace-nowrap",
    "group-data-[regen=true]:text-warcraft-gold/70",
    "group-data-[primary=true]:text-warcraft-gold",
    "group-data-[regen=true]:text-[clamp(1.3rem,0.85rem+0.32vw,1.75rem)]",
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
