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
    "[&_.grid-editor-tile[data-draggable=false]_.filled-tile]:cursor-default!",
    "[&_.grid-editor-tile[data-draggable=false]_.filled-tile]:opacity-[0.32]!",
    "[&_.grid-editor-tile[data-draggable=false]_.filled-tile]:[filter:saturate(0.35)_brightness(0.85)]!",
    "[&_.grid-editor-tile[data-draggable=false]_.filled-tile]:border-warcraft-blue-deep!",
    "[&_.grid-editor-tile[data-draggable=false]_.filled-tile]:shadow-bevel-hl!",
    "[&_.grid-editor-tile[data-draggable=false]_.filled-tile]:[transform:none]!",
    "[&_.grid-editor-tile[data-draggable=false]_.empty-tile:not([data-drop-target=true])]:[border:2px_dashed_color-mix(in_oklab,var(--color-warcraft-gold)_45%,transparent)]!",
    "[&_.grid-editor-tile[data-draggable=false]_.empty-tile:not([data-drop-target=true])]:[background:color-mix(in_oklab,var(--color-warcraft-gold)_4%,transparent)]!",
    "[&_.grid-editor-tile[data-draggable=false]:hover_.empty-tile:not([data-drop-target=true])]:border-warcraft-gold/75!",
    "[&_.grid-editor-tile[data-draggable=false]:hover_.empty-tile:not([data-drop-target=true])]:[background:color-mix(in_oklab,var(--color-warcraft-gold)_10%,transparent)]!",
    // During a drag every target STAYS golden-dashed (identical to its resting
    // look), and only gains a soft 12px glow — production does not turn the whole
    // board solid on lift. The single tile under the cursor goes solid gold with
    // the big glow via the `data-drag-over` rule below, which out-specifies this.
    "[&_.empty-tile[data-drop-target=true]]:[border:2px_dashed_color-mix(in_oklab,var(--color-warcraft-gold)_45%,transparent)]!",
    "[&_.empty-tile[data-drop-target=true]]:[background:color-mix(in_oklab,var(--color-warcraft-gold)_4%,transparent)]!",
    "[&_.empty-tile[data-drop-target=true]]:cursor-pointer!",
    "[&_.empty-tile[data-drop-target=true]]:shadow-glow-12-3!",
    "[&_.grid-editor-tile[data-drag-over=true]_.empty-tile]:[border:2px_solid_var(--color-warcraft-gold)]!",
    "[&_.grid-editor-tile[data-drag-over=true]_.empty-tile]:bg-panel-gold-diag-32-2!",
    "[&_.grid-editor-tile[data-drag-over=true]_.empty-tile]:shadow-inset-ring!",
    "[&_.grid-editor-tile[data-draggable=true]_.filled-tile]:cursor-grab!",
    "[&_.grid-editor-tile[data-draggable=true]_.filled-tile]:border-warcraft-gold!",
    "[&_.grid-editor-tile[data-draggable=true]_.filled-tile]:[transition:none]!",
    "[&_.grid-editor-tile[data-draggable=true]:active_.filled-tile]:cursor-grabbing!",
    // The lifted off button hides its painter (opacity-0, so the drag hit test can
    // still find it), so the Host — not the painter — paints its ghost. It reads as
    // a resting golden-dashed target while lifted, and goes solid gold with the big
    // glow the instant it is under the cursor, exactly like an empty target.
    "[&_.grid-editor-tile[data-draggable=true][data-dragging-source=true]]:[border:2px_dashed_color-mix(in_oklab,var(--color-warcraft-gold)_45%,transparent)]!",
    "[&_.grid-editor-tile[data-draggable=true][data-dragging-source=true]]:[background:color-mix(in_oklab,var(--color-warcraft-gold)_4%,transparent)]!",
    "[&_.grid-editor-tile[data-draggable=true][data-dragging-source=true]]:[box-shadow:none]!",
    "[&_.grid-editor-tile[data-draggable=true][data-dragging-source=true][data-drag-over=true]]:[border:2px_solid_var(--color-warcraft-gold)]!",
    "[&_.grid-editor-tile[data-draggable=true][data-dragging-source=true][data-drag-over=true]]:bg-panel-gold-diag-32-2!",
    "[&_.grid-editor-tile[data-draggable=true][data-dragging-source=true][data-drag-over=true]]:shadow-inset-ring!",
];

const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
