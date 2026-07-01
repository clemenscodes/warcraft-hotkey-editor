use crate::classes;

const BASE: &[&str] = &[
    "flex-1",
    "flex",
    "flex-col",
    "gap-5",
    "py-6",
    "px-5",
    "overflow-y-auto",
];
const MOBILE: &[&str] = &[
    "mobile:gap-4",
    "mobile:pt-5",
    "mobile:px-4",
    "mobile:pb-[max(1.25rem,env(safe-area-inset-bottom))]",
];
const TABLET: &[&str] = &[
    "tablet:gap-4",
    "tablet:pt-5",
    "tablet:px-4",
    "tablet:pb-[max(1.25rem,env(safe-area-inset-bottom))]",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
