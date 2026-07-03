use crate::{classes, states};
use warcraft_api::Race;

const BASE: &[&str] = &[
    "flex-1",
    "min-w-0",
    "min-h-[44px]",
    "px-[0.5rem]",
    "bg-[rgba(13,31,61,0.55)]",
    "border",
    "border-[#1f3d63]",
    "rounded-[8px]",
    "text-[#c0c8d4]",
    "font-friz-quadrata",
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
    "hover:bg-[rgba(30,60,95,0.7)]",
    "hover:text-white",
    "focus:outline-none",
    "kb-focus:border-white",
    "kb-focus:shadow-[0_0_0_2px_#fff]",
    "data-[active=true]:bg-[linear-gradient(135deg,rgba(45,80,130,0.95)_0%,rgba(20,45,80,0.95)_100%)]",
];

const MOBILE: &[&str] = &[
    "mobile:text-[clamp(11px,2.8vw,14px)]",
    "mobile:px-[0.35rem]",
    "mobile:h-[44px]",
    "mobile:leading-none",
];

const TABLET: &[&str] = &[
    "tablet:text-[clamp(11px,2.8vw,14px)]",
    "tablet:px-[0.35rem]",
    "tablet:h-[44px]",
    "tablet:leading-none",
];

const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}

// The active-race accent: the tab's hover border and its active border/text/glow all
// take the race colour, chosen directly from the race rather than a cascaded var.
const HUMAN: &[&str] = &[
    "hover:border-[color:#6aa1ff]",
    "data-[active=true]:border-[color:#6aa1ff]",
    "data-[active=true]:text-[color:#6aa1ff]",
    "data-[active=true]:shadow-[0_0_6px_rgba(106,161,255,0.3)]",
];
const NIGHTELF: &[&str] = &[
    "hover:border-[color:#5fdada]",
    "data-[active=true]:border-[color:#5fdada]",
    "data-[active=true]:text-[color:#5fdada]",
    "data-[active=true]:shadow-[0_0_6px_rgba(95,218,218,0.3)]",
];
const ORC: &[&str] = &[
    "hover:border-[color:#ff7a7a]",
    "data-[active=true]:border-[color:#ff7a7a]",
    "data-[active=true]:text-[color:#ff7a7a]",
    "data-[active=true]:shadow-[0_0_6px_rgba(255,122,122,0.3)]",
];
const UNDEAD: &[&str] = &[
    "hover:border-[color:#c79bff]",
    "data-[active=true]:border-[color:#c79bff]",
    "data-[active=true]:text-[color:#c79bff]",
    "data-[active=true]:shadow-[0_0_6px_rgba(199,155,255,0.3)]",
];
const NEUTRAL: &[&str] = &[
    "hover:border-[color:#ffce63]",
    "data-[active=true]:border-[color:#ffce63]",
    "data-[active=true]:text-[color:#ffce63]",
    "data-[active=true]:shadow-[0_0_6px_rgba(255,206,99,0.3)]",
];
states! {
    Race, Human => HUMAN, Nightelf => NIGHTELF, Orc => ORC, Undead => UNDEAD, Neutral =>
    NEUTRAL,
}
