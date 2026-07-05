use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "group",
    "flex",
    "items-center",
    "justify-between",
    "gap-[0.5rem]",
    "w-[8.25rem]",
    "px-[0.5rem]",
    "py-[0.25rem]",
    "whitespace-nowrap",
    "[background:linear-gradient(135deg,color-mix(in_oklab,var(--color-warcraft-gold-dark)_60%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-shadow)_60%,transparent)_100%)]",
    "border",
    "border-warcraft-gold/45",
    "rounded-[6px]",
    "text-warcraft-gold",
    "text-[1.15rem]",
    "uppercase",
    "tracking-[0.06em]",
    "[text-shadow:1px_1px_0_var(--color-warcraft-shadow)]",
    "cursor-pointer",
    "[transition:border-color_0.15s_ease,box-shadow_0.15s_ease]",
    "hover:border-warcraft-gold",
    "hover:[box-shadow:0_0_8px_color-mix(in_oklab,var(--color-warcraft-gold)_30%,transparent)]",
    "data-[open=true]:border-warcraft-gold",
    "data-[open=true]:[box-shadow:0_0_12px_color-mix(in_oklab,var(--color-warcraft-gold)_40%,transparent)]",
    "[body[data-kb-modality]_&]:focus:outline-none",
    "[body[data-kb-modality]_&]:focus:border-white",
    "[body[data-kb-modality]_&]:focus:text-white",
    "[body[data-kb-modality]_&]:focus:[box-shadow:0_0_0_2px_var(--color-warcraft-highlight),0_0_14px_color-mix(in_oklab,var(--color-warcraft-highlight)_45%,transparent)]",
];
const MOBILE: &[TailwindClass] = tw![
    "mobile:w-full",
    "mobile:px-[10px]",
    "mobile:py-[5px]",
    "mobile:min-h-[32px]",
    "mobile:text-[16px]",
    "mobile:tracking-[0.04em]",
];
const TABLET: &[TailwindClass] = tw![
    "tablet:w-full",
    "tablet:px-[10px]",
    "tablet:py-[5px]",
    "tablet:min-h-[32px]",
    "tablet:text-[14px]",
    "tablet:tracking-[0.04em]",
];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
