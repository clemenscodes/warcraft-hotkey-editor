use crate::{classes, styling::TailwindClass, tw};

// The search text field. A plain bordered box on the sidebar; on mobile it becomes a
// tall pill with room for the leading magnifier icon and a gold focus glow.
const BASE: &[TailwindClass] = tw![
    "flex-1",
    "min-w-0",
    "w-full",
    "bg-warcraft-bg-base/70",
    "border",
    "border-warcraft-blue",
    "rounded-[4px]",
    "text-white",
    "px-4",
    "py-3",
    "text-[1.4rem]",
    "focus:outline-none",
    "focus:border-warcraft-gold",
    "focus:shadow-[0_0_6px_color-mix(in_oklab,var(--color-warcraft-gold)_40%,transparent)]",
];

const MOBILE: &[TailwindClass] = tw![
    "mobile:h-[44px]",
    "mobile:pl-[40px]",
    "mobile:pr-[14px]",
    "mobile:py-0",
    "mobile:text-[16px]",
    "mobile:rounded-[10px]",
    "mobile:bg-[linear-gradient(180deg,color-mix(in_oklab,var(--color-warcraft-bg-base)_85%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-bg-mid)_85%,transparent)_100%)]",
    "mobile:border-warcraft-gold/45",
    "mobile:text-warcraft-text-primary",
    "mobile:tracking-[0.04em]",
    "mobile:shadow-[inset_0_1px_0_color-mix(in_oklab,var(--color-warcraft-highlight)_4%,transparent),0_1px_0_color-mix(in_oklab,var(--color-warcraft-shadow)_60%,transparent)]",
    "mobile:placeholder:text-warcraft-text-primary/50",
    "mobile:placeholder:italic",
    "mobile:focus:border-warcraft-gold",
    "mobile:focus:shadow-[0_0_0_2px_color-mix(in_oklab,var(--color-warcraft-gold)_35%,transparent),inset_0_1px_0_color-mix(in_oklab,var(--color-warcraft-highlight)_6%,transparent),0_0_14px_color-mix(in_oklab,var(--color-warcraft-gold)_30%,transparent)]",
    "mobile:[&::-webkit-search-cancel-button]:appearance-none",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:h-[44px]",
    "tablet:pl-[40px]",
    "tablet:pr-[14px]",
    "tablet:py-0",
    "tablet:text-[16px]",
    "tablet:rounded-[10px]",
    "tablet:bg-[linear-gradient(180deg,color-mix(in_oklab,var(--color-warcraft-bg-base)_85%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-bg-mid)_85%,transparent)_100%)]",
    "tablet:border-warcraft-gold/45",
    "tablet:text-warcraft-text-primary",
    "tablet:tracking-[0.04em]",
    "tablet:shadow-[inset_0_1px_0_color-mix(in_oklab,var(--color-warcraft-highlight)_4%,transparent),0_1px_0_color-mix(in_oklab,var(--color-warcraft-shadow)_60%,transparent)]",
    "tablet:placeholder:text-warcraft-text-primary/50",
    "tablet:placeholder:italic",
    "tablet:focus:border-warcraft-gold",
    "tablet:focus:shadow-[0_0_0_2px_color-mix(in_oklab,var(--color-warcraft-gold)_35%,transparent),inset_0_1px_0_color-mix(in_oklab,var(--color-warcraft-highlight)_6%,transparent),0_0_14px_color-mix(in_oklab,var(--color-warcraft-gold)_30%,transparent)]",
    "tablet:[&::-webkit-search-cancel-button]:appearance-none",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
