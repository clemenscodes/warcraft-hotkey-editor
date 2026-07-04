use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "flex",
    "flex-col",
    "self-start",
    "w-full",
    "min-w-0",
    "min-h-0",
    "max-h-full",
    "gap-[clamp(0.95rem,1.6vh,1.5rem)]",
    "p-[clamp(0.9rem,1.2vh,1.25rem)_clamp(1rem,1vw,1.5rem)]",
    "border",
    "border-[#1f3d63]",
    "rounded-[12px]",
    "bg-[linear-gradient(135deg,rgba(13,31,61,0.7)_0%,rgba(6,12,31,0.7)_100%)]",
    "overflow-hidden",
    "data-[empty=true]:items-center",
    "data-[empty=true]:justify-center",
    "data-[empty=true]:min-h-[16rem]",
    "data-[empty=true]:text-[#7b818d]",
    "data-[empty=true]:italic",
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
