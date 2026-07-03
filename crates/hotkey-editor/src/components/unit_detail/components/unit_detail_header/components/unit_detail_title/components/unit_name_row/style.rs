use crate::classes;

const BASE: &[&str] = &[
    "flex",
    "items-center",
    "justify-between",
    "gap-[clamp(0.75rem,0.8vw,1rem)]",
    "min-h-0",
];
const MOBILE: &[&str] = &[
    "mobile:flex-row",
    "mobile:items-center",
    "mobile:gap-[8px]",
    "mobile:w-full",
    "mobile:min-w-0",
];
const TABLET: &[&str] = &[
    "tablet:flex-row",
    "tablet:items-center",
    "tablet:gap-[8px]",
    "tablet:w-full",
    "tablet:min-w-0",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
