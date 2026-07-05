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
    "bg-[rgba(13,31,61,0.55)]",
    "border-[#1f3d63]",
    "text-[#e0d8c8]",
    "hover:bg-[rgba(30,60,95,0.7)]",
    "hover:text-white",
    "kb-focus:border-white",
    "kb-focus:text-white",
    "kb-focus:bg-[rgba(40,80,130,0.85)]",
    "kb-focus:shadow-[0_0_0_3px_#fff,0_0_16px_rgba(255,255,255,0.55)]",
    "data-[selected=true]:bg-[linear-gradient(135deg,rgba(45,80,130,0.9)_0%,rgba(20,45,80,0.9)_100%)]",
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
    "mobile:bg-[linear-gradient(180deg,rgba(13,31,61,0.55)_0%,rgba(8,14,30,0.55)_100%)]",
    "mobile:border-[rgba(42,80,133,0.6)]",
    "mobile:hover:border-[rgba(255,206,99,0.35)]",
    "mobile:data-[selected=true]:bg-[linear-gradient(135deg,rgba(45,80,130,0.85)_0%,rgba(20,45,80,0.85)_100%)]",
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
    "tablet:bg-[linear-gradient(180deg,rgba(13,31,61,0.55)_0%,rgba(8,14,30,0.55)_100%)]",
    "tablet:border-[rgba(42,80,133,0.6)]",
    "tablet:hover:border-[rgba(255,206,99,0.35)]",
    "tablet:data-[selected=true]:bg-[linear-gradient(135deg,rgba(45,80,130,0.85)_0%,rgba(20,45,80,0.85)_100%)]",
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
    "hover:border-[color:#6aa1ff]",
    "data-[selected=true]:border-[color:#6aa1ff]",
    "data-[selected=true]:text-[color:#6aa1ff]",
    "data-[selected=true]:shadow-[0_0_8px_rgba(106,161,255,0.3)]",
];
const NIGHTELF: &[TailwindClass] = tw![
    "hover:border-[color:#5fdada]",
    "data-[selected=true]:border-[color:#5fdada]",
    "data-[selected=true]:text-[color:#5fdada]",
    "data-[selected=true]:shadow-[0_0_8px_rgba(95,218,218,0.3)]",
];
const ORC: &[TailwindClass] = tw![
    "hover:border-[color:#ff7a7a]",
    "data-[selected=true]:border-[color:#ff7a7a]",
    "data-[selected=true]:text-[color:#ff7a7a]",
    "data-[selected=true]:shadow-[0_0_8px_rgba(255,122,122,0.3)]",
];
const UNDEAD: &[TailwindClass] = tw![
    "hover:border-[color:#c79bff]",
    "data-[selected=true]:border-[color:#c79bff]",
    "data-[selected=true]:text-[color:#c79bff]",
    "data-[selected=true]:shadow-[0_0_8px_rgba(199,155,255,0.3)]",
];
const NEUTRAL: &[TailwindClass] = tw![
    "hover:border-[color:#ffce63]",
    "data-[selected=true]:border-[color:#ffce63]",
    "data-[selected=true]:text-[color:#ffce63]",
    "data-[selected=true]:shadow-[0_0_8px_rgba(255,206,99,0.3)]",
];
states! {
    Race, Human => HUMAN, Nightelf => NIGHTELF, Orc => ORC, Undead => UNDEAD, Neutral =>
    NEUTRAL,
}
