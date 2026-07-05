use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
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
    "text-[1.8rem]/[1.45]",
    "whitespace-pre",
    "overflow-auto",
    "resize-y",
    "focus:outline-none",
    "focus:border-warcraft-gold",
    "focus:shadow-[0_0_8px_rgba(255,206,99,0.4)]",
];

const MOBILE: &[TailwindClass] = tw!["mobile:text-[1.4rem]/[1.45]"];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
