use super::ability_binding::AbilityBinding;
use super::command_binding::CommandBinding;
use super::system_binding::SystemBinding;

/// A fully-typed keybinding parsed from a single section of CustomKeys.txt.
#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum WarcraftKeybinding {
    /// Abilities, units, upgrades, and items — all non-command, non-system sections.
    Ability(AbilityBinding),
    /// Cmd* command sections (CmdAttack, CmdMove, …).
    Command(CommandBinding),
    /// System hotkey sections (inventory slots, hero selection, control groups, …).
    System(SystemBinding),
}

impl WarcraftKeybinding {
    pub fn as_ability(&self) -> Option<&AbilityBinding> {
        if let Self::Ability(binding) = self {
            Some(binding)
        } else {
            None
        }
    }

    pub fn as_ability_mut(&mut self) -> Option<&mut AbilityBinding> {
        if let Self::Ability(binding) = self {
            Some(binding)
        } else {
            None
        }
    }

    pub fn as_command(&self) -> Option<&CommandBinding> {
        if let Self::Command(binding) = self {
            Some(binding)
        } else {
            None
        }
    }

    pub fn as_command_mut(&mut self) -> Option<&mut CommandBinding> {
        if let Self::Command(binding) = self {
            Some(binding)
        } else {
            None
        }
    }

    pub fn as_system(&self) -> Option<&SystemBinding> {
        if let Self::System(binding) = self {
            Some(binding)
        } else {
            None
        }
    }

    pub fn as_system_mut(&mut self) -> Option<&mut SystemBinding> {
        if let Self::System(binding) = self {
            Some(binding)
        } else {
            None
        }
    }
}
