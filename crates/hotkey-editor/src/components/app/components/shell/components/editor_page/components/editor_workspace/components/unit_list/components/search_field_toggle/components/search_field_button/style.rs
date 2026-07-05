use crate::{classes, styling::TailwindClass, tw};

// One button of the Unit/Ability search-field toggle. Same bronze pill as the
// Melee/Campaign mode buttons; gold when active. Its height is set by the group's
// `[&>button]` rule, so only text and padding scale by viewport here.
const BASE: &[TailwindClass] = tw![
    "flex-1",
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
    "kb-focus:border-white",
    "kb-focus:text-white",
    "kb-focus:shadow-[0_0_0_3px_var(--color-warcraft-highlight),0_0_16px_color-mix(in_oklab,var(--color-warcraft-highlight)_55%,transparent)]",
    "data-[active=true]:bg-[linear-gradient(180deg,color-mix(in_oklab,var(--color-warcraft-gold)_18%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-gold-dark)_55%,transparent)_100%)]",
    "data-[active=true]:border-warcraft-gold",
    "data-[active=true]:text-warcraft-gold",
    "data-[active=true]:shadow-[0_0_12px_color-mix(in_oklab,var(--color-warcraft-gold)_30%,transparent)]",
];

const MOBILE: &[TailwindClass] = tw!["mobile:text-[1rem]", "mobile:px-[0.6rem]"];

const TABLET: &[TailwindClass] = tw![
    "tablet:text-[clamp(1rem,0.5vw+0.7rem,1.4rem)]",
    "tablet:px-4",
];

const LAPTOP: &[TailwindClass] = tw![
    "laptop:text-[clamp(1rem,0.5vw+0.7rem,1.4rem)]",
    "laptop:px-4",
];

const DESKTOP: &[TailwindClass] = tw![
    "desktop:text-[clamp(1rem,0.5vw+0.7rem,1.4rem)]",
    "desktop:px-4",
];

const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
