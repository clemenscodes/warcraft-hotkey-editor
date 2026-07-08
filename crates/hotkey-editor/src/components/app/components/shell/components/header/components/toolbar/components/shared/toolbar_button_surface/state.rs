/// The three resting looks a gold toolbar surface can wear. The dispatcher matches on
/// this to render the matching look component — `InteractiveSurface`,
/// `AttentionSurface`, or `ClearSurface`. Each look owns its own `<button>` root: the
/// chrome (box, border width, radius, resting gradient, focus ring) is shared as the
/// same utility-class values in every look's `style.rs`, and each layers its own text
/// color, resting border/glow, and hover treatment on top.
///
/// - `Interactive` — the default file-action look every inline toolbar button wears:
///   muted text at rest that brightens to gold on hover.
/// - `Attention` — a persistently gold surface, used when the button is surfacing a
///   condition that needs the user's eye (the collisions button while collisions remain).
/// - `Clear` — a gold-bordered surface with a soft resting glow, used for the affirmative
///   "all clear" look (the collisions button once the config is clean).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum SurfaceState {
    #[default]
    Interactive,
    Attention,
    Clear,
}
