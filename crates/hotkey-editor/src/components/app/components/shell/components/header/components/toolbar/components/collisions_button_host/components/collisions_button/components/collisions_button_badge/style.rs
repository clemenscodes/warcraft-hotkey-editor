use crate::{classes, styling::TailwindClass, tw};

// The count badge scales with the button: its corner offsets and font size are cqi
// fractions of the button box, so it holds its position and proportion at any size.
const BASE: &[TailwindClass] = tw![
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

const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
