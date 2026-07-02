use crate::classes;

const BASE: &[&str] = &[
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
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
