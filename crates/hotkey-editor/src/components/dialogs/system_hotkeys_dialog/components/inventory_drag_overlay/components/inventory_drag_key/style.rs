use crate::classes;

// The key glyph on the drag follower: the same gold Friz Quadrata cap a slot
// shows, at the inventory slot's size.
const BASE: &[&str] = &[
    "font-friz-quadrata",
    "uppercase",
    "tracking-[0.04em]",
    "text-[3.4rem]",
    "leading-none",
    "whitespace-nowrap",
    "text-warcraft-gold",
    "[text-shadow:1px_1px_0_#000,0_0_14px_rgba(255,206,99,0.45)]",
];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
