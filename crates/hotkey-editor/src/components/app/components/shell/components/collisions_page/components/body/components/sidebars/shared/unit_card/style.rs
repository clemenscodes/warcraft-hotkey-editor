use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
    "group",
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
    "hover:border-[color:var(--color-warcraft-blue)]",
    "hover:text-white",
    "kb-focus:border-white",
    "kb-focus:text-white",
    "kb-focus:bg-warcraft-blue/85",
    "kb-focus:focus-ring",
    "data-[selected=true]:bg-panel-blue-diag-90",
    "data-[selected=true]:border-warcraft-gold",
    "data-[selected=true]:text-warcraft-gold",
    "data-[selected=true]:shadow-glow-8-2",
];
const MOBILE: &[TailwindClass] = tw![
    "mobile:flex-[1_0_auto]",
    "mobile:w-[min(54vw,260px)]",
    "mobile:h-[clamp(96px,25vw,120px)]",
    "mobile:min-h-[clamp(96px,25vw,120px)]",
    "mobile:max-h-[clamp(96px,25vw,120px)]",
    "mobile:p-[8px_10px]",
    "mobile:gap-[10px]",
    "mobile:[scroll-snap-align:start]",
    "mobile:box-border",
    "mobile:overflow-hidden",
    "mobile:bg-panel-dark-55",
    "mobile:border-warcraft-blue/60",
];
const TABLET: &[TailwindClass] = tw![
    "tablet:flex-[1_0_auto]",
    "tablet:w-[min(54vw,260px)]",
    "tablet:h-[clamp(96px,25vw,120px)]",
    "tablet:min-h-[clamp(96px,25vw,120px)]",
    "tablet:max-h-[clamp(96px,25vw,120px)]",
    "tablet:p-[8px_10px]",
    "tablet:gap-[10px]",
    "tablet:[scroll-snap-align:start]",
    "tablet:box-border",
    "tablet:overflow-hidden",
    "tablet:bg-panel-dark-55",
    "tablet:border-warcraft-blue/60",
];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
