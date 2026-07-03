use crate::ApplicationLayer;
use crate::Layered;

/// Executes a command, in the strict-CQRS split where the command is inert data
/// and the handler is the code that carries it out.
///
/// In this crate a [`crate::Command`] already knows how to `execute` itself and
/// [`crate::Service::dispatch`] drives it — so a separate handler is optional. It
/// earns its place when execution needs collaborators a plain command should not
/// hold (a domain service, a clock, another repository), which the handler owns
/// while the command stays a pure request.
pub trait CommandHandler<TheCommand>: Layered<Layer = ApplicationLayer> {
    /// What handling the command reports back.
    type Outcome;

    /// Carries out the command.
    fn handle(&self, command: TheCommand) -> Self::Outcome;
}
