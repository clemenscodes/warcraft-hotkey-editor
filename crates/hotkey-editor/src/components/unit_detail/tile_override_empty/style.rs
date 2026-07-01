use crate::classes;

// The placeholder shown in the override panel before a grid tile is selected: a
// dashed muted-italic box.
const BASE: &[&str] = &[
    "flex",
    "flex-col",
    "items-center",
    "justify-center",
    "flex-[0_0_auto]",
    "overflow-hidden",
    "p-[2rem_2.25rem]",
    "border",
    "border-dashed",
    "border-[#4a7090]",
    "rounded-[10px]",
    "bg-[rgba(13,31,61,0.45)]",
    "text-[#7b818d]",
    "text-[1.8rem]",
    "leading-[1.45]",
    "italic",
];
const MOBILE: &[&str] = &[
    "mobile:w-full",
    "mobile:box-border",
    "mobile:h-[300px]",
    "mobile:p-[10px_12px]",
    "mobile:text-center",
    "mobile:rounded-[12px_12px_0_0]",
    "mobile:border-b-0",
    "mobile:shadow-[0_-4px_16px_rgba(0,0,0,0.4)]",
];
const TABLET: &[&str] = &[
    "tablet:w-full",
    "tablet:box-border",
    "tablet:h-[300px]",
    "tablet:p-[10px_12px]",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
