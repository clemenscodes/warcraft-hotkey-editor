use crate::{classes, styling::TailwindClass, tw};

// The Host coincides with the `TileFace` painter it wraps — same square, same corners —
// so the drag-over ring, dragging-source ghost, and focus ring sit exactly over the
// drawn tile. The query container itself lives on the painter (so its badge sizes with
// no Host, in the preview); the Host has none, so its own `cqi` overlays resolve against
// the outer grid, the same box they always did.
//
// The cursor tracks draggability: `grab` only on a draggable tile (`data-draggable=true`),
// the default arrow otherwise — an empty tile is not draggable, so it reads as inert — and
// `grabbing` while any drag is in flight.
const BASE: &[TailwindClass] = tw![
    "relative",
    "w-full",
    "aspect-square",
    "rounded-[1.04cqi]",
    "touch-pan-y",
    "cursor-default",
    "data-[draggable=true]:cursor-grab",
    "outline-none",
    "kb-focus:[box-shadow:0_0_0_0.52cqi_var(--color-warcraft-gold),0_0_3.1cqi_color-mix(in_oklab,var(--color-warcraft-gold)_55%,transparent)]",
    "data-[drag-over=true]:[box-shadow:0_0_0_0.35cqi_var(--color-warcraft-gold)]",
    "[body:has([data-dragging-source=true])_&]:cursor-grabbing",
    "data-[dragging-source=true]:[&>*]:invisible",
    "data-[dragging-source=true]:border-[0.35cqi]",
    "data-[dragging-source=true]:border-dashed",
    "data-[dragging-source=true]:border-warcraft-blue-bright",
    "data-[dragging-source=true]:[background:linear-gradient(135deg,color-mix(in_oklab,var(--color-warcraft-bg-mid)_85%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-bg-base)_85%,transparent)_100%)]",
    "data-[dragging-source=true]:[box-shadow:inset_0_1px_0_color-mix(in_oklab,var(--color-warcraft-highlight)_4%,transparent),0_1px_2px_color-mix(in_oklab,var(--color-warcraft-shadow)_50%,transparent)]",
    "data-[dragging-source=true]:data-[drag-over=true]:[box-shadow:0_0_0_0.35cqi_var(--color-warcraft-gold)]",
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
