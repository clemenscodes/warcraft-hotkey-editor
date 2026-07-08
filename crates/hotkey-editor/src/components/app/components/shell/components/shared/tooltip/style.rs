use tw_macro::tw;

// The tooltip bubble, positioned against its `group/tooltip relative` trigger. It
// reveals only while the trigger is hovered or keyboard-focused, via the named
// `group/tooltip` on the trigger.
//
// Positioning has two mutually-exclusive bands, split on `@supports (anchor-name)`:
//   - anchor positioning (the `supports-[…]` band): the bubble is tethered to its
//     trigger's `--tooltip-anchor` and lifted to `fixed`, so it escapes a scrolling
//     or clipping ancestor (e.g. the tall key-picker dialog body) instead of being
//     cut off; `position-try-fallbacks` keeps it on-screen. `data-placement=above`
//     flips it above the trigger.
//   - the absolute fallback (the `not-supports-[…]` band): below-center by default,
//     `data-placement=above` flips it above, and `data-anchor=left/right` pin it to
//     an edge so an edge tooltip stays on-screen.
classes! {
    base: tw![
        "pointer-events-none",
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
        "supports-[anchor-name:--a]:fixed",
        "supports-[anchor-name:--a]:[position-anchor:--tooltip-anchor]",
        "supports-[anchor-name:--a]:[position-area:block-end]",
        "supports-[anchor-name:--a]:[position-try-fallbacks:flip-block,flip-inline]",
        "supports-[anchor-name:--a]:[margin:0.6rem_0]",
        "supports-[anchor-name:--a]:data-[placement=above]:[position-area:block-start]",
        "not-supports-[anchor-name:--a]:absolute",
        "not-supports-[anchor-name:--a]:top-[calc(100%+0.6rem)]",
        "not-supports-[anchor-name:--a]:left-1/2",
        "not-supports-[anchor-name:--a]:-translate-x-1/2",
        "not-supports-[anchor-name:--a]:data-[placement=above]:top-auto",
        "not-supports-[anchor-name:--a]:data-[placement=above]:bottom-[calc(100%+0.6rem)]",
        "not-supports-[anchor-name:--a]:data-[anchor=left]:left-0",
        "not-supports-[anchor-name:--a]:data-[anchor=left]:right-auto",
        "not-supports-[anchor-name:--a]:data-[anchor=left]:translate-x-0",
        "not-supports-[anchor-name:--a]:data-[anchor=right]:left-auto",
        "not-supports-[anchor-name:--a]:data-[anchor=right]:right-0",
        "not-supports-[anchor-name:--a]:data-[anchor=right]:translate-x-0",
    ],
}
