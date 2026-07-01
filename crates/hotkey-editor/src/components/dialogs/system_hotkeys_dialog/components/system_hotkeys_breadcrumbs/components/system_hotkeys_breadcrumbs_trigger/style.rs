use crate::classes;

// The dropdown trigger: hidden on desktop (the tab bar shows instead); on small
// viewports a full-width gold pill showing the active category.
const BASE: &[&str] = &[
    "hidden",
    "[body[data-kb-modality]_&]:focus-visible:outline-none",
    "[body[data-kb-modality]_&]:focus-visible:border-white",
    "[body[data-kb-modality]_&]:focus-visible:text-white",
    "[body[data-kb-modality]_&]:focus-visible:[box-shadow:0_0_0_2px_#fff,0_0_16px_rgba(255,255,255,0.55)]",
];
const MOBILE: &[&str] = &[
    "mobile:flex",
    "mobile:items-center",
    "mobile:justify-between",
    "mobile:w-full",
    "mobile:min-h-[44px]",
    "mobile:py-[0.55rem]",
    "mobile:px-[0.9rem]",
    "mobile:border",
    "mobile:border-[rgba(255,206,99,0.55)]",
    "mobile:rounded-[8px]",
    "mobile:cursor-pointer",
    "mobile:uppercase",
    "mobile:font-friz-quadrata",
    "mobile:text-warcraft-gold",
    "mobile:text-[clamp(14px,3.8vw,17px)]",
    "mobile:tracking-[0.06em]",
    "mobile:[background:linear-gradient(135deg,rgba(40,30,8,0.85)_0%,rgba(15,12,4,0.85)_100%)]",
    "mobile:[text-shadow:1px_1px_0_rgba(0,0,0,0.92)]",
    "mobile:[box-shadow:0_0_14px_rgba(255,206,99,0.18)]",
];
const TABLET: &[&str] = &[
    "tablet:flex",
    "tablet:items-center",
    "tablet:justify-between",
    "tablet:w-full",
    "tablet:min-h-[44px]",
    "tablet:py-[0.55rem]",
    "tablet:px-[0.9rem]",
    "tablet:border",
    "tablet:border-[rgba(255,206,99,0.55)]",
    "tablet:rounded-[8px]",
    "tablet:cursor-pointer",
    "tablet:uppercase",
    "tablet:font-friz-quadrata",
    "tablet:text-warcraft-gold",
    "tablet:text-[clamp(14px,3.8vw,17px)]",
    "tablet:tracking-[0.06em]",
    "tablet:[background:linear-gradient(135deg,rgba(40,30,8,0.85)_0%,rgba(15,12,4,0.85)_100%)]",
    "tablet:[text-shadow:1px_1px_0_rgba(0,0,0,0.92)]",
    "tablet:[box-shadow:0_0_14px_rgba(255,206,99,0.18)]",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
