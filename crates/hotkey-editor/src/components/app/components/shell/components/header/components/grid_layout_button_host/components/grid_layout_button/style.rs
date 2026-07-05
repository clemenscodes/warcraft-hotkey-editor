use crate::{classes, styling::TailwindClass, tw};

// Fills its host container and draws itself as one cqi-scaled drawing: it takes the host's
// full box (`size-full`) — the host owns the `aspect-[39/10]` shape and gets its size from
// the header — and expresses every interior length — padding, gap, border, radius, font,
// glow — as a `cqi` fraction of the host box. Make the host taller and the whole button
// scales up in proportion; there is no fixed length left inside. The header sizes the host
// off the shared row height, so the button grows coherently from laptop through 4K.
const BASE: &[TailwindClass] = tw![
    "inline-flex",
    "items-center",
    "justify-center",
    "gap-[4.27cqi]",
    "size-full",
    "px-[8.55cqi]",
    "border-[0.3cqi]",
    "border-warcraft-gold",
    "rounded-[3.2cqi]",
    "text-warcraft-gold",
    "text-[8.55cqi]",
    "tracking-[0.14em]",
    "font-medium",
    "cursor-pointer",
    "[background:linear-gradient(135deg,color-mix(in_oklab,var(--color-warcraft-gold-dark)_85%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-shadow)_85%,transparent)_100%)]",
    "[box-shadow:0_0_5.88cqi_color-mix(in_oklab,var(--color-warcraft-gold)_22%,transparent)]",
    "[transition:background_0.12s_ease,box-shadow_0.12s_ease,transform_0.12s_ease]",
    "focus:outline-none",
    "focus-visible:border-white",
    "focus-visible:text-white",
    "focus-visible:[box-shadow:0_0_0_0.8cqi_var(--color-warcraft-highlight),0_0_4.81cqi_color-mix(in_oklab,var(--color-warcraft-highlight)_55%,transparent)]",
    "hover:[background:linear-gradient(135deg,color-mix(in_oklab,var(--color-warcraft-gold)_22%,transparent)_0%,color-mix(in_oklab,var(--color-race-neutral-strong)_95%,transparent)_100%)]",
    "hover:[box-shadow:0_0_6.94cqi_color-mix(in_oklab,var(--color-warcraft-gold)_55%,transparent),inset_0_0_3.74cqi_color-mix(in_oklab,var(--color-warcraft-gold)_15%,transparent)]",
];

const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
