use dioxus::prelude::*;

/// The editor page's route parameters: the race, unit mode, selected unit, and search
/// query the URL carries (`/?race=&mode=&unit=&q=`). The page reconciles them into the
/// shell's navigation signals and reads every other piece of editor state — the loaded
/// document, the grid layout, the selection and drag machinery — from context, so it is
/// no longer fed a god-bag of signals as props.
#[derive(Props, Clone, PartialEq)]
pub struct EditorPageProps {
    pub race: Option<String>,
    pub mode: Option<String>,
    pub unit: Option<String>,
    pub q: Option<String>,
}
