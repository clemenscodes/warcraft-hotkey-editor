use crate::{classes, states, styling::TailwindClass, tw};
use warcraft_api::Race;

const BASE: &[TailwindClass] = tw![
    "flex",
    "items-center",
    "gap-4",
    "p-4",
    "w-full",
    "min-w-0",
    "text-left",
    "text-[1.4rem]",
    "tracking-[0.02em]",
    "border",
    "rounded-[6px]",
    "transition-all",
    "duration-[0.12s]",
    "bg-warcraft-bg-mid/55",
    "border-warcraft-blue-deep",
    "text-warcraft-text-primary",
    "hover:bg-warcraft-blue-deep/70",
    "hover:text-white",
    "kb-focus:border-white",
    "kb-focus:text-white",
    "kb-focus:bg-warcraft-blue/85",
    "kb-focus:shadow-[0_0_0_3px_var(--color-warcraft-highlight),0_0_16px_color-mix(in_oklab,var(--color-warcraft-highlight)_55%,transparent)]",
    "data-[selected=true]:bg-[linear-gradient(135deg,color-mix(in_oklab,var(--color-warcraft-blue)_90%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-bg-panel)_90%,transparent)_100%)]",
];

const MOBILE: &[TailwindClass] = tw![
    "mobile:flex-[1_0_auto]",
    "mobile:flex-row",
    "mobile:justify-start",
    "mobile:w-[min(54vw,260px)]",
    "mobile:h-[clamp(96px,25vw,120px)]",
    "mobile:min-h-[clamp(96px,25vw,120px)]",
    "mobile:max-h-[clamp(96px,25vw,120px)]",
    "mobile:p-[8px_10px]",
    "mobile:gap-[10px]",
    "mobile:[scroll-snap-align:start]",
    "mobile:box-border",
    "mobile:overflow-hidden",
    "mobile:bg-[linear-gradient(180deg,color-mix(in_oklab,var(--color-warcraft-bg-mid)_55%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-bg-base)_55%,transparent)_100%)]",
    "mobile:border-warcraft-blue/60",
    "mobile:hover:border-warcraft-gold/35",
    "mobile:data-[selected=true]:bg-[linear-gradient(135deg,color-mix(in_oklab,var(--color-warcraft-blue)_85%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-bg-panel)_85%,transparent)_100%)]",
    "mobile:group-[[data-search-active=false][data-active-category=hero]]:[&:not([data-unit-kind=hero])]:hidden",
    "mobile:group-[[data-search-active=false][data-active-category=soldier]]:[&:not([data-unit-kind=soldier])]:hidden",
    "mobile:group-[[data-search-active=false][data-active-category=worker]]:[&:not([data-unit-kind=worker])]:hidden",
    "mobile:group-[[data-search-active=false][data-active-category=building]]:[&:not([data-unit-kind=building])]:hidden",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:flex-[1_0_auto]",
    "tablet:flex-row",
    "tablet:justify-start",
    "tablet:w-[min(54vw,260px)]",
    "tablet:h-[clamp(96px,25vw,120px)]",
    "tablet:min-h-[clamp(96px,25vw,120px)]",
    "tablet:max-h-[clamp(96px,25vw,120px)]",
    "tablet:p-[8px_10px]",
    "tablet:gap-[10px]",
    "tablet:[scroll-snap-align:start]",
    "tablet:box-border",
    "tablet:overflow-hidden",
    "tablet:bg-[linear-gradient(180deg,color-mix(in_oklab,var(--color-warcraft-bg-mid)_55%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-bg-base)_55%,transparent)_100%)]",
    "tablet:border-warcraft-blue/60",
    "tablet:hover:border-warcraft-gold/35",
    "tablet:data-[selected=true]:bg-[linear-gradient(135deg,color-mix(in_oklab,var(--color-warcraft-blue)_85%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-bg-panel)_85%,transparent)_100%)]",
    "tablet:group-[[data-search-active=false][data-active-category=hero]]:[&:not([data-unit-kind=hero])]:hidden",
    "tablet:group-[[data-search-active=false][data-active-category=soldier]]:[&:not([data-unit-kind=soldier])]:hidden",
    "tablet:group-[[data-search-active=false][data-active-category=worker]]:[&:not([data-unit-kind=worker])]:hidden",
    "tablet:group-[[data-search-active=false][data-active-category=building]]:[&:not([data-unit-kind=building])]:hidden",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}

// The active-race accent: the hover border (desktop; mobile/tablet keep their own
// gold hover from the bands) and the selected border/text/glow all take the race
// colour, chosen directly from the race rather than a cascaded var. The selected
// glow is one blur for every width (the former 10px mobile/tablet variant folds into
// this 8px — a 2px difference that no longer justifies a band-specific race style).
const HUMAN: &[TailwindClass] = tw![
    "hover:border-[color:var(--color-race-human)]",
    "data-[selected=true]:border-[color:var(--color-race-human)]",
    "data-[selected=true]:text-[color:var(--color-race-human)]",
    "data-[selected=true]:shadow-[0_0_8px_color-mix(in_oklab,var(--color-race-human)_30%,transparent)]",
];
const NIGHTELF: &[TailwindClass] = tw![
    "hover:border-[color:var(--color-race-nightelf)]",
    "data-[selected=true]:border-[color:var(--color-race-nightelf)]",
    "data-[selected=true]:text-[color:var(--color-race-nightelf)]",
    "data-[selected=true]:shadow-[0_0_8px_color-mix(in_oklab,var(--color-race-nightelf)_30%,transparent)]",
];
const ORC: &[TailwindClass] = tw![
    "hover:border-[color:var(--color-race-orc)]",
    "data-[selected=true]:border-[color:var(--color-race-orc)]",
    "data-[selected=true]:text-[color:var(--color-race-orc)]",
    "data-[selected=true]:shadow-[0_0_8px_color-mix(in_oklab,var(--color-race-orc)_30%,transparent)]",
];
const UNDEAD: &[TailwindClass] = tw![
    "hover:border-[color:var(--color-race-undead)]",
    "data-[selected=true]:border-[color:var(--color-race-undead)]",
    "data-[selected=true]:text-[color:var(--color-race-undead)]",
    "data-[selected=true]:shadow-[0_0_8px_color-mix(in_oklab,var(--color-race-undead)_30%,transparent)]",
];
const NEUTRAL: &[TailwindClass] = tw![
    "hover:border-[color:var(--color-warcraft-gold)]",
    "data-[selected=true]:border-[color:var(--color-warcraft-gold)]",
    "data-[selected=true]:text-[color:var(--color-warcraft-gold)]",
    "data-[selected=true]:shadow-[0_0_8px_color-mix(in_oklab,var(--color-warcraft-gold)_30%,transparent)]",
];
states! {
    Race, Human => HUMAN, Nightelf => NIGHTELF, Orc => ORC, Undead => UNDEAD, Neutral =>
    NEUTRAL,
}
