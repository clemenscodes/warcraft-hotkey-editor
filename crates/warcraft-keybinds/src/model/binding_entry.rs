use super::ability_binding::AbilityBinding;
use crate::identity::ability_id::AbilityId;
use std::ops::Deref;

#[derive(Clone, Copy, Debug)]
pub struct BindingEntry<'a> {
    ability_id: AbilityId,
    binding: &'a AbilityBinding,
}

impl<'a> BindingEntry<'a> {
    pub(crate) fn new(ability_id: AbilityId, binding: &'a AbilityBinding) -> Self {
        Self {
            ability_id,
            binding,
        }
    }

    pub fn ability_id(&self) -> AbilityId {
        self.ability_id
    }

    pub fn binding(&self) -> &'a AbilityBinding {
        self.binding
    }
}

impl<'a> Deref for BindingEntry<'a> {
    type Target = AbilityBinding;

    fn deref(&self) -> &AbilityBinding {
        self.binding
    }
}
