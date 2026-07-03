use crate::classes;

const BASE: &[&str] = &[
    "flex",
    "items-center",
    "justify-center",
    "w-9",
    "h-9",
    "rounded-md",
    "border-0",
    "bg-transparent",
    "text-warcraft-text-muted",
    "cursor-pointer",
    "transition-[color,background-color,border-color]",
    "duration-150",
    "text-[2rem]",
    "leading-none",
    "hover:text-warcraft-gold",
];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
