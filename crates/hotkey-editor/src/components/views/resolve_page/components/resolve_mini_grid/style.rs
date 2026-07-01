use crate::classes;
const BASE: &[&str] = &[
    "flex-[1_1_auto]",
    "w-full",
    "max-w-[260px]",
    "min-w-0",
    "aspect-[4/3]",
    "grid",
    "grid-cols-[repeat(4,1fr)]",
    "grid-rows-[repeat(3,1fr)]",
    "gap-[3px]",
    "p-[4px]",
    "bg-[rgba(20,35,60,0.7)]",
    "border",
    "border-[#2a5085]",
    "rounded-[4px]",
];
const MOBILE: &[&str] = &["mobile:max-w-[max(130px,32vw)]"];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
