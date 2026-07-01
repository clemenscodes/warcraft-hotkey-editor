use crate::classes;

const BASE: &[&str] = &[
    "flex",
    "flex-col",
    "[grid-column:2/3]",
    "self-start",
    "w-full",
    "min-w-0",
    "min-h-0",
    "max-h-full",
    "p-[clamp(0.9rem,1.2vh,1.25rem)_clamp(1rem,1vw,1.5rem)]",
    "pb-[clamp(1rem,1.5vh,1.5rem)]",
    "gap-[clamp(0.95rem,1.6vh,1.5rem)]",
    "border",
    "border-[#1f3d63]",
    "rounded-[12px]",
    "bg-[linear-gradient(135deg,rgba(13,31,61,0.7)_0%,rgba(6,12,31,0.7)_100%)]",
    "shadow-[inset_0_1px_0_rgba(255,255,255,0.04)]",
    "overflow-y-auto",
    "overflow-x-clip",
    "[overscroll-behavior:contain]",
    "[scrollbar-width:thin]",
    "[scrollbar-color:rgba(255,206,99,0.35)_transparent]",
    "[&::-webkit-scrollbar]:w-[5px]",
    "[&::-webkit-scrollbar-track]:bg-transparent",
    "[&::-webkit-scrollbar-thumb]:bg-[rgba(255,206,99,0.35)]",
    "[&::-webkit-scrollbar-thumb]:rounded-[3px]",
    "[&::-webkit-scrollbar-thumb:hover]:bg-[rgba(255,206,99,0.55)]",
];
const MOBILE: &[&str] = &[
    "mobile:self-auto",
    "mobile:max-h-none",
    "mobile:p-[0.85rem]",
    "mobile:rounded-[6px]",
    "mobile:overflow-visible",
];
const TABLET: &[&str] = &[
    "tablet:self-stretch",
    "tablet:max-h-none",
    "tablet:p-8",
    "tablet:overflow-visible",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
