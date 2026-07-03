use crate::classes;

// The blue-edged block that describes an ability's off-state or upgraded form. Shared
// by the alt-state and upgrade sections.
const BASE: &[&str] = &[
    "flex",
    "flex-col",
    "gap-[0.3rem]",
    "py-[0.7rem]",
    "pr-0",
    "pl-4",
    "bg-[rgba(8,18,35,0.55)]",
    "border-l-2",
    "border-[#6ec3ff]",
    "rounded-[4px_0_0_4px]",
    "text-[#c8d4ec]",
    "text-[1.4rem]",
    "leading-[1.5]",
];

const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
