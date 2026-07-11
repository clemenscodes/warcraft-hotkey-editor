use tw_macro::tw;
// On laptop and up the bar is a three-column grid (brand | centered layout button |
// toolbar). Its `min-height` is a fixed per-band step (`min-h-18` → `24` → `28` → `34`
// from laptop through 4K) — one truth per band, nothing inherited across bands. Its inner
// rhythm — column `gap`, vertical `py` — is `cqi` off the bar itself, so it tracks the
// bar's own width, never the raw viewport. Everything *inside* the bar likewise scales in
// `cqi`: `items-stretch` gives every column the full row height so the layout button and
// toolbar buttons fill it and render at one shared height.
// Below laptop it collapses to a flex row (brand left, toolbar right) sized for touch.
//
// The horizontal inset is one always-on `px-4`, so the bar's content is indented by the
// same amount on every band. Padding never clips an element's background, so the bar's
// fill (its own gradient below laptop, the app's fixed backdrop above) stays edge-to-edge
// while only the content is inset. Vertical padding stays band-specific: safe-area top +
// `pb-2` below laptop, symmetric `vw` on laptop and up.
//
// The gold divider that separates the bar from the app is the header's own `::after`, not
// a full-width `border-b`: it is absolutely positioned along the bottom and inset
// `left-4 right-4` to the same `px-4` edge as the content, so the line ends exactly where
// the content ends while the bar's fill still spans edge-to-edge. The double drop-shadow
// under it is the original bevel, now riding the inset line.
//
// The header is always a query container (`@container` in BASE): every `cqi` length in its
// children — the brand on every band, the layout button and toolbar buttons on laptop up —
// resolves against the bar's own width, not the raw viewport. `container-type` makes a
// *query* container only; unlike `contain`/`transform` it is not a containing block, so the
// mobile burger's `fixed` drawer still fills the screen.

// From laptop up the bar carries its height as a fixed per-band `min-height` step
// (`min-h-18` → `24` → `28` → `34`), stepping up once per band above the mobile/tablet
// `min-h-14`. `items-stretch` hands that same row height to every column, so the layout
// button and the toolbar buttons fill it and render at one shared height. `py-[0.7cqi]`
// then insets that fill so the buttons sit shorter than the bar with breathing room above
// and below — this padding, `cqi` off the bar, is the one knob for the button-to-bar height
// ratio. No button carries a fixed size — change the per-band `min-height` (bar height) or
// the `py` (button ratio) and the whole bar rescales together within the band.

classes! {
    base: tw![
        "@container",
        "relative",
        "z-50",
        "items-center",
        "flex-none",
        "px-4",
        "after:content-['']",
        "after:absolute",
        "after:bottom-0",
        "after:left-4",
        "after:right-4",
        "after:h-px",
        "after:bg-warcraft-gold/40",
        "after:shadow-edge",
    ],
    mobile: tw![
        "mobile:flex",
        "mobile:flex-row",
        "mobile:justify-between",
        "mobile:sticky",
        "mobile:top-0",
        "mobile:z-60",
        "mobile:pt-2",
        "mobile:pb-2",
        "mobile:after:bg-warcraft-gold/30",
        "mobile:min-h-14",
        "mobile:max-w-full",
        "mobile:w-full",
        "mobile:bg-warcraft-bg-base",
        "mobile:bg-panel-header",
        "mobile:bg-no-repeat",
        "mobile:bg-fixed",
        "mobile:bg-size-[100%_100%]",
    ],
    tablet: tw![
        "tablet:flex",
        "tablet:flex-row",
        "tablet:justify-between",
        "tablet:sticky",
        "tablet:top-0",
        "tablet:z-60",
        "tablet:pt-2",
        "tablet:pb-2",
        "tablet:after:bg-warcraft-gold/30",
        "tablet:min-h-14",
        "tablet:max-w-full",
        "tablet:w-full",
        "tablet:bg-warcraft-bg-base",
        "tablet:bg-panel-header",
        "tablet:bg-no-repeat",
        "tablet:bg-fixed",
        "tablet:bg-size-[100%_100%]",
    ],
    laptop: tw![
        "laptop:grid",
        "laptop:grid-cols-[minmax(0,1fr)_auto_minmax(0,1fr)]",
        "laptop:items-stretch",
        "laptop:min-h-18",
        "laptop:gap-[1cqi]",
        "laptop:py-[0.7cqi]",
    ],
    desktop: tw![
        "desktop:grid",
        "desktop:grid-cols-[minmax(0,1fr)_auto_minmax(0,1fr)]",
        "desktop:items-stretch",
        "desktop:min-h-24",
        "desktop:gap-[1cqi]",
        "desktop:py-[0.7cqi]",
    ],
    qhd: tw![
        "qhd:grid",
        "qhd:grid-cols-[minmax(0,1fr)_auto_minmax(0,1fr)]",
        "qhd:items-stretch",
        "qhd:min-h-28",
        "qhd:gap-[1cqi]",
        "qhd:py-[0.7cqi]",
    ],
    uhd: tw![
        "uhd:grid",
        "uhd:grid-cols-[minmax(0,1fr)_auto_minmax(0,1fr)]",
        "uhd:items-stretch",
        "uhd:min-h-34",
        "uhd:gap-[1cqi]",
        "uhd:py-[0.7cqi]",
    ],
}
