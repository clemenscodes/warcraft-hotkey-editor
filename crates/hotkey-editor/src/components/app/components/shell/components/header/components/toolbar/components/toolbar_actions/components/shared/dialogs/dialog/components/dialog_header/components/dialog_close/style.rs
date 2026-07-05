use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "absolute",
    "right-[1rem]",
    "top-1/2",
    "-translate-y-1/2",
    "w-[2.5rem]",
    "h-[2.5rem]",
    "text-[1.5rem]",
    "flex",
    "items-center",
    "justify-center",
    "bg-transparent",
    "border-0",
    "cursor-pointer",
    "text-warcraft-text-secondary",
    "[text-shadow:1px_1px_0_var(--color-warcraft-shadow)]",
    "transition-[color,text-shadow]",
    "duration-150",
    "hover:text-warcraft-gold",
    "hover:[text-shadow:1px_1px_0_var(--color-warcraft-shadow),0_0_12px_color-mix(in_oklab,var(--color-warcraft-gold)_55%,transparent)]",
    "focus:outline-none",
    "kb-focus:text-white",
    "kb-focus:[text-shadow:1px_1px_0_var(--color-warcraft-shadow),0_0_16px_color-mix(in_oklab,var(--color-warcraft-highlight)_70%,transparent)]",
];

const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
