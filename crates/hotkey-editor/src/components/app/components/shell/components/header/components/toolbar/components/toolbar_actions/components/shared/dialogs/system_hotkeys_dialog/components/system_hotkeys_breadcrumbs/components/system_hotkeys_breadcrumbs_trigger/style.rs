use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "hidden",
    "[body[data-kb-modality]_&]:focus-visible:outline-none",
    "[body[data-kb-modality]_&]:focus-visible:border-white",
    "[body[data-kb-modality]_&]:focus-visible:text-white",
    "[body[data-kb-modality]_&]:focus-visible:[box-shadow:0_0_0_2px_var(--color-warcraft-highlight),0_0_16px_color-mix(in_oklab,var(--color-warcraft-highlight)_55%,transparent)]",
];

const MOBILE: &[TailwindClass] = tw![
    "mobile:flex",
    "mobile:items-center",
    "mobile:justify-between",
    "mobile:w-full",
    "mobile:min-h-[44px]",
    "mobile:py-[0.55rem]",
    "mobile:px-[0.9rem]",
    "mobile:border",
    "mobile:border-warcraft-gold/55",
    "mobile:rounded-[8px]",
    "mobile:cursor-pointer",
    "mobile:uppercase",
    "mobile:text-warcraft-gold",
    "mobile:text-[clamp(14px,3.8vw,17px)]",
    "mobile:tracking-[0.06em]",
    "mobile:[background:linear-gradient(135deg,color-mix(in_oklab,var(--color-warcraft-gold-dark)_85%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-shadow)_85%,transparent)_100%)]",
    "mobile:[text-shadow:1px_1px_0_color-mix(in_oklab,var(--color-warcraft-shadow)_92%,transparent)]",
    "mobile:[box-shadow:0_0_14px_color-mix(in_oklab,var(--color-warcraft-gold)_18%,transparent)]",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:flex",
    "tablet:items-center",
    "tablet:justify-between",
    "tablet:w-full",
    "tablet:min-h-[44px]",
    "tablet:py-[0.55rem]",
    "tablet:px-[0.9rem]",
    "tablet:border",
    "tablet:border-warcraft-gold/55",
    "tablet:rounded-[8px]",
    "tablet:cursor-pointer",
    "tablet:uppercase",
    "tablet:text-warcraft-gold",
    "tablet:text-[clamp(14px,3.8vw,17px)]",
    "tablet:tracking-[0.06em]",
    "tablet:[background:linear-gradient(135deg,color-mix(in_oklab,var(--color-warcraft-gold-dark)_85%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-shadow)_85%,transparent)_100%)]",
    "tablet:[text-shadow:1px_1px_0_color-mix(in_oklab,var(--color-warcraft-shadow)_92%,transparent)]",
    "tablet:[box-shadow:0_0_14px_color-mix(in_oklab,var(--color-warcraft-gold)_18%,transparent)]",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
