use crate::classes;

const BASE: &[&str] = &[
    "h-[80px]",
    "w-[calc(80px/3*4)]",
    "shrink-0",
    "grid",
    "grid-cols-[repeat(4,1fr)]",
    "grid-rows-[repeat(3,1fr)]",
    "gap-[2px]",
    "p-[3px]",
    "bg-[rgba(20,35,60,0.7)]",
    "border",
    "border-warcraft-blue",
    "rounded-[3px]",
];
const MOBILE: &[&str] = &["mobile:h-[66px]", "mobile:w-[calc(66px/3*4)]"];
const TABLET: &[&str] = &["tablet:h-[92px]", "tablet:w-[calc(92px/3*4)]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
