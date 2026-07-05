use crate::{classes, states, styling::TailwindClass, tw};
use warcraft_api::Race;

const BASE: &[TailwindClass] = tw![
    "flex-1",
    "min-w-0",
    "min-h-[44px]",
    "px-[0.5rem]",
    "bg-warcraft-bg-mid/55",
    "border",
    "border-warcraft-blue-deep",
    "rounded-[8px]",
    "text-warcraft-text-secondary",
    "text-[0.95rem]",
    "tracking-[0.04em]",
    "uppercase",
    "text-center",
    "cursor-pointer",
    "transition-all",
    "duration-[0.12s]",
    "whitespace-nowrap",
    "overflow-hidden",
    "text-ellipsis",
    "hover:bg-warcraft-blue-deep/70",
    "hover:text-white",
    "focus:outline-none",
    "kb-focus:border-white",
    "kb-focus:shadow-[0_0_0_2px_var(--color-warcraft-highlight)]",
    "data-[active=true]:bg-[linear-gradient(135deg,color-mix(in_oklab,var(--color-warcraft-blue)_95%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-bg-panel)_95%,transparent)_100%)]",
];

const MOBILE: &[TailwindClass] = tw![
    "mobile:text-[clamp(11px,2.8vw,14px)]",
    "mobile:px-[0.35rem]",
    "mobile:h-[44px]",
    "mobile:leading-none",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:text-[clamp(11px,2.8vw,14px)]",
    "tablet:px-[0.35rem]",
    "tablet:h-[44px]",
    "tablet:leading-none",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}

// The active-race accent: the tab's hover border and its active border/text/glow all
// take the race colour, chosen directly from the race rather than a cascaded var.
const HUMAN: &[TailwindClass] = tw![
    "hover:border-[color:var(--color-race-human)]",
    "data-[active=true]:border-[color:var(--color-race-human)]",
    "data-[active=true]:text-[color:var(--color-race-human)]",
    "data-[active=true]:shadow-[0_0_6px_color-mix(in_oklab,var(--color-race-human)_30%,transparent)]",
];
const NIGHTELF: &[TailwindClass] = tw![
    "hover:border-[color:var(--color-race-nightelf)]",
    "data-[active=true]:border-[color:var(--color-race-nightelf)]",
    "data-[active=true]:text-[color:var(--color-race-nightelf)]",
    "data-[active=true]:shadow-[0_0_6px_color-mix(in_oklab,var(--color-race-nightelf)_30%,transparent)]",
];
const ORC: &[TailwindClass] = tw![
    "hover:border-[color:var(--color-race-orc)]",
    "data-[active=true]:border-[color:var(--color-race-orc)]",
    "data-[active=true]:text-[color:var(--color-race-orc)]",
    "data-[active=true]:shadow-[0_0_6px_color-mix(in_oklab,var(--color-race-orc)_30%,transparent)]",
];
const UNDEAD: &[TailwindClass] = tw![
    "hover:border-[color:var(--color-race-undead)]",
    "data-[active=true]:border-[color:var(--color-race-undead)]",
    "data-[active=true]:text-[color:var(--color-race-undead)]",
    "data-[active=true]:shadow-[0_0_6px_color-mix(in_oklab,var(--color-race-undead)_30%,transparent)]",
];
const NEUTRAL: &[TailwindClass] = tw![
    "hover:border-[color:var(--color-warcraft-gold)]",
    "data-[active=true]:border-[color:var(--color-warcraft-gold)]",
    "data-[active=true]:text-[color:var(--color-warcraft-gold)]",
    "data-[active=true]:shadow-[0_0_6px_color-mix(in_oklab,var(--color-warcraft-gold)_30%,transparent)]",
];
states! {
    Race, Human => HUMAN, Nightelf => NIGHTELF, Orc => ORC, Undead => UNDEAD, Neutral =>
    NEUTRAL,
}
