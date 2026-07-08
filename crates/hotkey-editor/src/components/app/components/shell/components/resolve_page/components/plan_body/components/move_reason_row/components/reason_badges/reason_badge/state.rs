/// The colour a reason badge wears. Each maps to one race/status palette; the
/// per-kind wrappers pick the right one (Fight and Stuck both wear Orc).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ReasonBadgeColor {
    Orc,
    Human,
    Undead,
    Success,
}
