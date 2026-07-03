use crate::classes;

const BASE: &[&str] = &[
    "w-[clamp(5.25rem,4.3vw,7rem)]",
    "h-[clamp(5.25rem,4.3vw,7rem)]",
    "[image-rendering:auto]",
    "border-2",
    "border-warcraft-blue",
    "rounded-[4px]",
    "[box-shadow:0_0_6px_rgba(0,0,0,0.5)]",
    "object-cover",
    "bg-[rgba(20,35,60,0.7)]",
    "text-transparent",
    "text-[0]",
    "leading-[0]",
];
const MOBILE: &[&str] = &[
    "mobile:shrink-0",
    "mobile:self-start",
    "mobile:w-[clamp(80px,22vw,120px)]",
    "mobile:h-[clamp(80px,22vw,120px)]",
];
const TABLET: &[&str] = &[
    "tablet:shrink-0",
    "tablet:self-start",
    "tablet:w-[clamp(80px,22vw,120px)]",
    "tablet:h-[clamp(80px,22vw,120px)]",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
