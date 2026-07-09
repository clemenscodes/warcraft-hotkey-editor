use super::components::active_race_tab::ActiveRaceTabProps;
use super::components::inactive_race_tab::InactiveRaceTabProps;
use super::hooks::RaceTabBehavior;

impl From<&RaceTabBehavior> for ActiveRaceTabProps {
    fn from(behavior: &RaceTabBehavior) -> Self {
        let label = behavior.label().clone();
        let onclick = behavior.onclick();
        let onkeydown = behavior.onkeydown();
        let onmounted = behavior.onmounted();
        Self {
            label,
            onclick,
            onkeydown,
            onmounted,
        }
    }
}

impl From<&RaceTabBehavior> for InactiveRaceTabProps {
    fn from(behavior: &RaceTabBehavior) -> Self {
        let label = behavior.label().clone();
        let onclick = behavior.onclick();
        let onkeydown = behavior.onkeydown();
        let onmounted = behavior.onmounted();
        Self {
            label,
            onclick,
            onkeydown,
            onmounted,
        }
    }
}
