use tw_macro::tw;

// The GapPull reason's flavour: it publishes only its accent colour as `--reason-color`
// and disappears from layout (`contents`) so the composed `ReasonBadge` pill it wraps is
// the inline element. It never names the pill's classes.
classes! {
    base: tw![
        "contents",
        "[--reason-color:var(--color-warcraft-success)]",
    ],
}
