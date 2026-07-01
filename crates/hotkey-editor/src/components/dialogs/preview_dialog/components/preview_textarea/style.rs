use crate::classes;

const BASE: &[&str] = &[
    "w-full",
    "flex-1",
    "min-h-[20rem]",
    "px-8",
    "py-6",
    "rounded-md",
    "border",
    "border-warcraft-blue",
    "bg-[rgba(8,18,35,0.85)]",
    "text-warcraft-text-primary",
    "font-mono",
    "text-[1.8rem]/[1.45]",
    "whitespace-pre",
    "overflow-auto",
    "resize-y",
    "focus:outline-none",
    "focus:border-warcraft-gold",
    "focus:shadow-[0_0_8px_rgba(255,206,99,0.4)]",
];

const MOBILE: &[&str] = &["mobile:text-[1.4rem]/[1.45]"];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
