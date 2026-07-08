/// A card's id text is muted grey normally; when its card is selected it takes the
/// active race color at reduced opacity. Chosen from the card's selected flag.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum UnitCardIdState {
    #[default]
    Normal,
    Selected,
}
