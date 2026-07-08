use tw_macro::ClassList;

/// The detail pane surface (shared value across the three detail panes); the
/// `data-empty` variants center and mute the empty-prompt state.
pub(super) const DETAIL: ClassList = ClassList::new(
    "flex flex-col self-stretch w-full min-w-0 min-h-0 max-h-full gap-6 py-4 px-5 border border-warcraft-blue-deep rounded-container bg-panel-dark overflow-hidden data-[empty=true]:items-center data-[empty=true]:justify-center data-[empty=true]:min-h-64 data-[empty=true]:text-warcraft-text-faint data-[empty=true]:italic",
);
/// The pane header row above the conflict grid.
pub(super) const HEADER: ClassList =
    ClassList::new("flex items-center gap-4 flex-none pb-3.5 border-b border-b-warcraft-gold/25");
/// The scrolling conflict-card grid.
pub(super) const GRID: ClassList = ClassList::new(
    "grid grid-cols-[repeat(auto-fill,minmax(450px,1fr))] gap-6 flex-[1_1_0] min-h-0 overflow-y-auto content-start pt-4 pr-3 pb-4 pl-0 scrollbar-thin [scrollbar-color:color-mix(in_oklab,var(--color-warcraft-gold)_45%,transparent)_transparent] mobile:grid-cols-[minmax(0,1fr)] mobile:flex-none mobile:min-h-auto mobile:overflow-y-visible mobile:py-4 mobile:px-0 tablet:grid-cols-[minmax(0,1fr)] tablet:flex-none tablet:min-h-auto tablet:overflow-y-visible tablet:py-4 tablet:px-0",
);
