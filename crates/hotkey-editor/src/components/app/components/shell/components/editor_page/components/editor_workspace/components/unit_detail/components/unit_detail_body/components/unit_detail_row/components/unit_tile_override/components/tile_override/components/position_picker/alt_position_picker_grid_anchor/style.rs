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
    "[&_.filled-tile[data-draggable=false]]:shadow-bevel-hl!",
    "[&_.filled-tile[data-draggable=false]]:[transform:none]!",
    "[&_.empty-tile[data-draggable=false]]:[border:2px_dashed_color-mix(in_oklab,var(--color-warcraft-gold)_45%,transparent)]!",
    "[&_.empty-tile[data-draggable=false]]:[background:color-mix(in_oklab,var(--color-warcraft-gold)_4%,transparent)]!",
    "[&_.empty-tile[data-draggable=false]]:hover:border-warcraft-gold/75!",
    "[&_.empty-tile[data-draggable=false]]:hover:[background:color-mix(in_oklab,var(--color-warcraft-gold)_10%,transparent)]!",
    "[&_.empty-tile[data-drop-target=true]]:[border:2px_solid_color-mix(in_oklab,var(--color-warcraft-gold)_75%,transparent)]!",
    "[&_.empty-tile[data-drop-target=true]]:[background:color-mix(in_oklab,var(--color-warcraft-gold)_14%,transparent)]!",
    "[&_.empty-tile[data-drop-target=true]]:cursor-pointer!",
    "[&_.empty-tile[data-drop-target=true]]:shadow-glow-12-3!",
    "[&_.empty-tile[data-drag-over=true]]:[border:2px_solid_var(--color-warcraft-gold)]!",
    "[&_.empty-tile[data-drag-over=true]]:bg-panel-gold-diag-32-2!",
    "[&_.empty-tile[data-drag-over=true]]:shadow-inset-ring!",
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
    "[&_.filled-tile[data-draggable=true][data-dragging-source=true][data-drag-over=true]]:bg-panel-gold-diag-32-2!",
    "[&_.filled-tile[data-draggable=true][data-dragging-source=true][data-drag-over=true]]:shadow-inset-ring!",
];

const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
