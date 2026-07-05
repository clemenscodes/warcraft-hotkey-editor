use crate::{classes, styling::TailwindClass, tw};

// Centers the embedded command grid inside a position-picker dialog and restyles its
// tiles for the picker's single-button drag: non-draggable tiles dim out,
// drop-target and drag-over cells glow gold. All descendant overrides are `!` so they
// win over the grid editor's own tile styling.
const BASE: &[TailwindClass] = tw![
    "flex",
    "justify-center",
    "w-full",
    "[&_.grid-section]:[container-type:normal]",
    "[&_.grid-section]:w-max",
    "[&_.grid]:[--tile-size:8rem]",
    "[&_.grid]:grid-cols-[repeat(4,var(--tile-size))]",
    "[&_.grid]:[grid-auto-rows:var(--tile-size)]",
    "[&_.filled-tile]:w-[var(--tile-size)]",
    "[&_.filled-tile]:h-[var(--tile-size)]",
    "[&_.empty-tile]:w-[var(--tile-size)]",
    "[&_.empty-tile]:h-[var(--tile-size)]",
    "[&_.command-tile-wrapper]:w-[var(--tile-size)]",
    "[&_.command-tile-wrapper]:h-[var(--tile-size)]",
    "[&_.filled-tile[data-draggable=false]]:cursor-default!",
    "[&_.filled-tile[data-draggable=false]]:opacity-[0.32]!",
    "[&_.filled-tile[data-draggable=false]]:[filter:saturate(0.35)_brightness(0.85)]!",
    "[&_.filled-tile[data-draggable=false]]:border-warcraft-blue-deep!",
    "[&_.filled-tile[data-draggable=false]]:[box-shadow:inset_0_1px_0_color-mix(in_oklab,var(--color-warcraft-highlight)_4%,transparent),0_1px_2px_color-mix(in_oklab,var(--color-warcraft-shadow)_50%,transparent)]!",
    "[&_.filled-tile[data-draggable=false]]:[transform:none]!",
    "[&_.empty-tile[data-draggable=false]]:[border:2px_dashed_color-mix(in_oklab,var(--color-warcraft-gold)_45%,transparent)]!",
    "[&_.empty-tile[data-draggable=false]]:[background:color-mix(in_oklab,var(--color-warcraft-gold)_4%,transparent)]!",
    "[&_.empty-tile[data-draggable=false]]:hover:border-warcraft-gold/75!",
    "[&_.empty-tile[data-draggable=false]]:hover:[background:color-mix(in_oklab,var(--color-warcraft-gold)_10%,transparent)]!",
    "[&_.empty-tile[data-drop-target=true]]:[border:2px_solid_color-mix(in_oklab,var(--color-warcraft-gold)_75%,transparent)]!",
    "[&_.empty-tile[data-drop-target=true]]:[background:color-mix(in_oklab,var(--color-warcraft-gold)_14%,transparent)]!",
    "[&_.empty-tile[data-drop-target=true]]:cursor-pointer!",
    "[&_.empty-tile[data-drop-target=true]]:[box-shadow:0_0_12px_color-mix(in_oklab,var(--color-warcraft-gold)_25%,transparent)]!",
    "[&_.empty-tile[data-drag-over=true]]:[border:2px_solid_var(--color-warcraft-gold)]!",
    "[&_.empty-tile[data-drag-over=true]]:[background:linear-gradient(135deg,color-mix(in_oklab,var(--color-warcraft-gold)_32%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-gold)_18%,transparent)_100%),color-mix(in_oklab,var(--color-warcraft-bg-panel)_95%,transparent)]!",
    "[&_.empty-tile[data-drag-over=true]]:[box-shadow:inset_0_0_0_1px_color-mix(in_oklab,var(--color-warcraft-gold)_65%,transparent),0_0_28px_color-mix(in_oklab,var(--color-warcraft-gold)_85%,transparent)]!",
    "[&_.filled-tile[data-draggable=true]]:cursor-grab!",
    "[&_.filled-tile[data-draggable=true]]:border-warcraft-gold!",
    "[&_.filled-tile[data-draggable=true]]:[transition:none]!",
    "[&_.filled-tile[data-draggable=true]]:active:cursor-grabbing!",
    "[&_.filled-tile[data-draggable=true][data-dragging-source=true]]:[border:2px_dashed_color-mix(in_oklab,var(--color-warcraft-gold)_45%,transparent)]!",
    "[&_.filled-tile[data-draggable=true][data-dragging-source=true]]:[background:color-mix(in_oklab,var(--color-warcraft-gold)_4%,transparent)]!",
    "[&_.filled-tile[data-draggable=true][data-dragging-source=true]]:[box-shadow:none]!",
    "[&_.filled-tile[data-draggable=true][data-dragging-source=true]]:[transform:none]!",
    "[&_.filled-tile[data-draggable=true][data-dragging-source=true]]:[filter:none]!",
    "[&_.filled-tile[data-draggable=true][data-dragging-source=true][data-drag-over=true]]:[border:3px_solid_var(--color-warcraft-gold)]!",
    "[&_.filled-tile[data-draggable=true][data-dragging-source=true][data-drag-over=true]]:[background:linear-gradient(135deg,color-mix(in_oklab,var(--color-warcraft-gold)_32%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-gold)_18%,transparent)_100%),color-mix(in_oklab,var(--color-warcraft-bg-panel)_95%,transparent)]!",
    "[&_.filled-tile[data-draggable=true][data-dragging-source=true][data-drag-over=true]]:[box-shadow:inset_0_0_0_1px_color-mix(in_oklab,var(--color-warcraft-gold)_65%,transparent),0_0_28px_color-mix(in_oklab,var(--color-warcraft-gold)_85%,transparent)]!",
];

const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
