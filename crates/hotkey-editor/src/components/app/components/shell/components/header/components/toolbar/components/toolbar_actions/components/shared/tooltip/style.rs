use tw_macro::tw;

// The tooltip bubble, positioned against its `group/tooltip relative` trigger.
// BASE is below-center; `data-placement=above` flips it above the trigger and the
// `data-anchor=left/right` variants pin it to an edge (so an edge key's tooltip
// stays on-screen). It reveals only while the trigger is hovered or keyboard-
// focused, via the named `group/tooltip` on the trigger.
classes! {
    base: tw![
        "pointer-events-none",
        "absolute",
        "top-[calc(100%+0.6rem)]",
        "left-1/2",
        "-translate-x-1/2",
        "z-1200",
        "w-max",
        "max-w-[38rem]",
        "px-4",
        "py-3",
        "border",
        "border-warcraft-gold",
        "rounded-control",
        "text-xl",
        "leading-body",
        "text-center",
        "whitespace-normal",
        "text-warcraft-gold",
        "bg-panel-dark",
        "text-shadow-drop",
        "shadow-raised",
        "opacity-0",
        "transition-opacity",
        "duration-base",
        "delay-[400ms]",
        "group-hover/tooltip:opacity-100",
        "group-focus-visible/tooltip:opacity-100",
        "data-[placement=above]:top-auto",
        "data-[placement=above]:bottom-[calc(100%+0.6rem)]",
        "data-[anchor=left]:left-0",
        "data-[anchor=left]:right-auto",
        "data-[anchor=left]:translate-x-0",
        "data-[anchor=right]:left-auto",
        "data-[anchor=right]:right-0",
        "data-[anchor=right]:translate-x-0",
    ],
}
