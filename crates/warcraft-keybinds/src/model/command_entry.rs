use super::command_binding::CommandBinding;
use std::ops::Deref;
use warcraft_api::WarcraftObjectId;

#[derive(Clone, Copy, Debug)]
pub struct CommandEntry<'a> {
    name: WarcraftObjectId,
    binding: &'a CommandBinding,
}

impl<'a> CommandEntry<'a> {
    pub(crate) fn new(name: WarcraftObjectId, binding: &'a CommandBinding) -> Self {
        Self { name, binding }
    }

    pub fn name(&self) -> WarcraftObjectId {
        self.name
    }

    pub fn binding(&self) -> &'a CommandBinding {
        self.binding
    }
}

impl<'a> Deref for CommandEntry<'a> {
    type Target = CommandBinding;

    fn deref(&self) -> &CommandBinding {
        self.binding
    }
}
