use tw_macro::tw;
// The attention resting look of a gold toolbar button: the chrome shared by every
// surface look — the box, the resting gradient, the hairline border, the radius, and
// the focus ring — drawn in `cqi` off the container the parent hands it, so the whole
// button scales as one drawing. On top of that chrome this look layers a persistently
// gold text that brightens on hover, for when the button is surfacing a condition that
// needs the user's eye. The border is tuned per band to read as a ~1px hairline:
// 2.8cqi on the compact 36px phone/tablet box, and 1.25cqi on the laptop-and-up box.

classes! {
    base: tw![
        "inline-flex",
        "items-center",
        "justify-center",
        "size-full",
        "p-0",
        "border-[1.25cqi]",
        "border-warcraft-gold-border",
        "rounded-[15cqi]",
        "cursor-pointer",
        "bg-panel-gold-resting",
        "transition-[border-color,color,background,box-shadow]",
        "focus:outline-none",
        "kb-focus:border-white",
        "kb-focus:text-white",
        "kb-focus:[--focus-color:var(--color-warcraft-highlight)]",
        "kb-focus:shadow-focus",
        "text-warcraft-gold",
        "hover:border-warcraft-gold",
        "hover:text-warcraft-gold",
        "hover:bg-panel-gold-active",
        "hover:shadow-glow",
    ],
    mobile: tw![
        "mobile:border-[2.8cqi]",
    ],
    tablet: tw![
        "tablet:border-[2.8cqi]",
    ],
}
