use crate::{classes, styling::TailwindClass, tw};

// One button of the catalog-visibility toggle (No abilities / All variants). A
// smaller bronze pill than the search-field toggle; gold when active. Height is set
// by the group's `[&>button]` rule.
const BASE: &[TailwindClass] = tw![
    "flex-1",
    "px-3",
    "whitespace-nowrap",
    "bg-[linear-gradient(180deg,rgba(40,30,8,0.55)_0%,rgba(15,12,4,0.55)_100%)]",
    "border",
    "border-[#6c5a1f]",
    "rounded-[8px]",
    "text-[#c0c8da]",
    "font-friz-quadrata",
    "text-[1.1rem]",
    "uppercase",
    "tracking-[0.08em]",
    "[text-shadow:1px_1px_0_#000]",
    "transition-[border-color,color,box-shadow]",
    "duration-150",
    "hover:border-warcraft-gold",
    "hover:text-warcraft-gold",
    "focus:outline-none",
    "kb-focus:border-white",
    "kb-focus:text-white",
    "kb-focus:shadow-[0_0_0_3px_#fff,0_0_16px_rgba(255,255,255,0.55)]",
    "data-[active=true]:bg-[linear-gradient(180deg,rgba(255,206,99,0.18)_0%,rgba(40,30,8,0.55)_100%)]",
    "data-[active=true]:border-warcraft-gold",
    "data-[active=true]:text-warcraft-gold",
    "data-[active=true]:shadow-[0_0_12px_rgba(255,206,99,0.3)]",
];

const MOBILE: &[TailwindClass] = tw!["mobile:text-[0.85rem]", "mobile:px-[0.5rem]"];

const TABLET: &[TailwindClass] = tw![
    "tablet:text-[clamp(0.8rem,0.45vw+0.55rem,1.05rem)]",
    "tablet:px-2",
];

const LAPTOP: &[TailwindClass] = tw![
    "laptop:text-[clamp(0.8rem,0.45vw+0.55rem,1.05rem)]",
    "laptop:px-2",
];

const DESKTOP: &[TailwindClass] = tw![
    "desktop:text-[clamp(0.8rem,0.45vw+0.55rem,1.05rem)]",
    "desktop:px-2",
];

const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
