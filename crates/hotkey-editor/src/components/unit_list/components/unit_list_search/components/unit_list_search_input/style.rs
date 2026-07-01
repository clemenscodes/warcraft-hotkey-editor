use crate::classes;

// The search text field. A plain bordered box on the sidebar; on mobile it becomes a
// tall pill with room for the leading magnifier icon and a gold focus glow.
const BASE: &[&str] = &[
    "flex-1",
    "min-w-0",
    "w-full",
    "bg-[rgba(8,18,35,0.7)]",
    "border",
    "border-[#2a5085]",
    "rounded-[4px]",
    "text-white",
    "px-4",
    "py-3",
    "font-[inherit]",
    "text-[1.4rem]",
    "focus:outline-none",
    "focus:border-warcraft-gold",
    "focus:shadow-[0_0_6px_rgba(255,206,99,0.4)]",
];

const MOBILE: &[&str] = &[
    "mobile:h-[44px]",
    "mobile:pl-[40px]",
    "mobile:pr-[14px]",
    "mobile:py-0",
    "mobile:text-[16px]",
    "mobile:rounded-[10px]",
    "mobile:bg-[linear-gradient(180deg,rgba(8,14,30,0.85)_0%,rgba(13,31,61,0.85)_100%)]",
    "mobile:border-[rgba(255,206,99,0.45)]",
    "mobile:text-[#fff5d6]",
    "mobile:font-friz-quadrata",
    "mobile:tracking-[0.04em]",
    "mobile:shadow-[inset_0_1px_0_rgba(255,255,255,0.04),0_1px_0_rgba(0,0,0,0.6)]",
    "mobile:placeholder:text-[rgba(255,245,214,0.5)]",
    "mobile:placeholder:italic",
    "mobile:focus:border-warcraft-gold",
    "mobile:focus:shadow-[0_0_0_2px_rgba(255,206,99,0.35),inset_0_1px_0_rgba(255,255,255,0.06),0_0_14px_rgba(255,206,99,0.3)]",
    "mobile:[&::-webkit-search-cancel-button]:appearance-none",
];

const TABLET: &[&str] = &[
    "tablet:h-[44px]",
    "tablet:pl-[40px]",
    "tablet:pr-[14px]",
    "tablet:py-0",
    "tablet:text-[16px]",
    "tablet:rounded-[10px]",
    "tablet:bg-[linear-gradient(180deg,rgba(8,14,30,0.85)_0%,rgba(13,31,61,0.85)_100%)]",
    "tablet:border-[rgba(255,206,99,0.45)]",
    "tablet:text-[#fff5d6]",
    "tablet:font-friz-quadrata",
    "tablet:tracking-[0.04em]",
    "tablet:shadow-[inset_0_1px_0_rgba(255,255,255,0.04),0_1px_0_rgba(0,0,0,0.6)]",
    "tablet:placeholder:text-[rgba(255,245,214,0.5)]",
    "tablet:placeholder:italic",
    "tablet:focus:border-warcraft-gold",
    "tablet:focus:shadow-[0_0_0_2px_rgba(255,206,99,0.35),inset_0_1px_0_rgba(255,255,255,0.06),0_0_14px_rgba(255,206,99,0.3)]",
    "tablet:[&::-webkit-search-cancel-button]:appearance-none",
];

const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
