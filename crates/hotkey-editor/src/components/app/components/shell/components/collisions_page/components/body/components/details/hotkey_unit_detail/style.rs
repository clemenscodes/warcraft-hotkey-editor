use tw_macro::ClassList;

/// The detail pane surface (shared value across the three detail panes); the
/// `data-empty` variants center and mute the empty-prompt state.
pub(super) const DETAIL: ClassList = ClassList::new(
    "flex flex-col self-stretch w-full min-w-0 min-h-0 max-h-full gap-6 py-4 px-5 border border-warcraft-blue-deep rounded-container bg-panel-dark overflow-hidden data-[empty=true]:items-center data-[empty=true]:justify-center data-[empty=true]:min-h-64 data-[empty=true]:text-warcraft-text-faint data-[empty=true]:italic",
);
