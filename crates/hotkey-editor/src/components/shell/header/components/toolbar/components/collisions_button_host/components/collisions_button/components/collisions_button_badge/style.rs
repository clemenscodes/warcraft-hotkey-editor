use crate::classes;

// The count badge scales with the button: its corner offsets and font size are cqi
// fractions of the button box, so it holds its position and proportion at any size.
const BASE: &[&str] = &[
    "absolute",
    "top-[8cqi]",
    "right-[9cqi]",
    "font-mono",
    "font-bold",
    "leading-none",
    "text-[length:20cqi]",
    "text-[#ffe39a]",
    "pointer-events-none",
    "[text-shadow:1.25cqi_1.25cqi_0_rgba(0,0,0,0.95),-1.25cqi_1.25cqi_0_rgba(0,0,0,0.95),1.25cqi_-1.25cqi_0_rgba(0,0,0,0.95),-1.25cqi_-1.25cqi_0_rgba(0,0,0,0.95),0_0_3.75cqi_rgba(0,0,0,0.95)]",
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
