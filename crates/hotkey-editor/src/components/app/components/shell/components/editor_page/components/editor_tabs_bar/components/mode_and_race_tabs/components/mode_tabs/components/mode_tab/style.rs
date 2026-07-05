use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "flex-1",
    "min-h-[2.75rem]",
    "px-6",
    "bg-[linear-gradient(180deg,color-mix(in_oklab,var(--color-warcraft-gold-dark)_55%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-shadow)_55%,transparent)_100%)]",
    "border",
    "border-warcraft-gold-border",
    "rounded-[10px]",
    "text-warcraft-text-secondary",
    "text-[1.8rem]",
    "uppercase",
    "tracking-[0.08em]",
    "[text-shadow:1px_1px_0_var(--color-warcraft-shadow)]",
    "transition-[border-color,color,box-shadow]",
    "duration-150",
    "hover:border-warcraft-gold",
    "hover:text-warcraft-gold",
    "focus:outline-none",
    "[body[data-kb-modality]_&]:focus:outline-none",
    "[body[data-kb-modality]_&]:focus:border-white",
    "[body[data-kb-modality]_&]:focus:text-white",
    "[body[data-kb-modality]_&]:focus:shadow-[0_0_0_3px_var(--color-warcraft-highlight),0_0_16px_color-mix(in_oklab,var(--color-warcraft-highlight)_55%,transparent)]",
    "data-[active=true]:bg-[linear-gradient(180deg,color-mix(in_oklab,var(--color-warcraft-gold)_18%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-gold-dark)_55%,transparent)_100%)]",
    "data-[active=true]:border-warcraft-gold",
    "data-[active=true]:text-warcraft-gold",
    "data-[active=true]:shadow-[0_0_12px_color-mix(in_oklab,var(--color-warcraft-gold)_30%,transparent)]",
];
const MOBILE: &[TailwindClass] = tw![
    "mobile:min-h-[3.2rem]",
    "mobile:text-[1rem]",
    "mobile:px-[0.6rem]",
];
const TABLET: &[TailwindClass] = tw![
    "tablet:text-[clamp(1rem,0.5vw+0.7rem,1.4rem)]",
    "tablet:px-4",
];
const LAPTOP: &[TailwindClass] = tw![
    "laptop:text-[clamp(1rem,0.5vw+0.7rem,1.4rem)]",
    "laptop:px-4",
];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
