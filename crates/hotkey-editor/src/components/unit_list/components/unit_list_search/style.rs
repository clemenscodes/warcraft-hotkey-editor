use crate::classes;

// The search box wrapper. On the sidebar it is a bordered inset panel; on mobile it
// becomes a bare relative box (the input carries its own chrome there) so the icon
// can be positioned over the field.
const BASE: &[&str] = &[
    "flex-none",
    "flex",
    "items-center",
    "gap-2",
    "p-2",
    "bg-[rgba(13,31,61,0.85)]",
    "border",
    "border-[#1f3d63]",
    "rounded-[6px]",
    "min-w-0",
];

const MOBILE: &[&str] = &[
    "mobile:relative",
    "mobile:p-0",
    "mobile:bg-transparent",
    "mobile:border-0",
    "mobile:rounded-none",
    "mobile:mb-[6px]",
];

const TABLET: &[&str] = &[
    "tablet:relative",
    "tablet:p-0",
    "tablet:bg-transparent",
    "tablet:border-0",
    "tablet:rounded-none",
    "tablet:mb-[6px]",
];

const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
