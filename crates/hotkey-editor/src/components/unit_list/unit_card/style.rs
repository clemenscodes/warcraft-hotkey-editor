use warcraft_api::UnitKind;

use crate::classes;

// A selectable unit in the list. On the sidebar it is a full-width row (icon | text);
// selected takes the inherited race color. In the mobile carousel it becomes a
// fixed-size snap tile. The race color is inherited from `.main-content[data-race]`
// via `var(--race-color)`.
const BASE: &[&str] = &[
    "flex",
    "items-center",
    "gap-4",
    "p-4",
    "w-full",
    "min-w-0",
    "text-left",
    "font-friz-quadrata",
    "text-[1.4rem]",
    "tracking-[0.02em]",
    "border",
    "rounded-[6px]",
    "transition-all",
    "duration-[0.12s]",
    "bg-[rgba(13,31,61,0.55)]",
    "border-[#1f3d63]",
    "text-[#e0d8c8]",
    "hover:bg-[rgba(30,60,95,0.7)]",
    "hover:border-[color:var(--race-color,#2a5085)]",
    "hover:text-white",
    "kb-focus:border-white",
    "kb-focus:text-white",
    "kb-focus:bg-[rgba(40,80,130,0.85)]",
    "kb-focus:shadow-[0_0_0_3px_#fff,0_0_16px_rgba(255,255,255,0.55)]",
    "data-[selected=true]:bg-[linear-gradient(135deg,rgba(45,80,130,0.9)_0%,rgba(20,45,80,0.9)_100%)]",
    "data-[selected=true]:border-[color:var(--race-color,#ffce63)]",
    "data-[selected=true]:text-[color:var(--race-color,#ffce63)]",
    "data-[selected=true]:shadow-[0_0_8px_var(--race-color-soft,rgba(255,206,99,0.3))]",
];
const MOBILE: &[&str] = &[
    "mobile:flex-[1_0_auto]",
    "mobile:flex-row",
    "mobile:justify-start",
    "mobile:w-[min(54vw,260px)]",
    "mobile:h-[clamp(96px,25vw,120px)]",
    "mobile:min-h-[clamp(96px,25vw,120px)]",
    "mobile:max-h-[clamp(96px,25vw,120px)]",
    "mobile:p-[8px_10px]",
    "mobile:gap-[10px]",
    "mobile:[scroll-snap-align:start]",
    "mobile:box-border",
    "mobile:overflow-hidden",
    "mobile:bg-[linear-gradient(180deg,rgba(13,31,61,0.55)_0%,rgba(8,14,30,0.55)_100%)]",
    "mobile:border-[rgba(42,80,133,0.6)]",
    "mobile:hover:border-[rgba(255,206,99,0.35)]",
    "mobile:data-[selected=true]:bg-[linear-gradient(135deg,rgba(45,80,130,0.85)_0%,rgba(20,45,80,0.85)_100%)]",
    "mobile:data-[selected=true]:border-[color:var(--race-color,#ffce63)]",
    "mobile:data-[selected=true]:shadow-[0_0_10px_var(--race-color-soft,rgba(255,206,99,0.3))]",
];
const TABLET: &[&str] = &[
    "tablet:flex-[1_0_auto]",
    "tablet:flex-row",
    "tablet:justify-start",
    "tablet:w-[min(54vw,260px)]",
    "tablet:h-[clamp(96px,25vw,120px)]",
    "tablet:min-h-[clamp(96px,25vw,120px)]",
    "tablet:max-h-[clamp(96px,25vw,120px)]",
    "tablet:p-[8px_10px]",
    "tablet:gap-[10px]",
    "tablet:[scroll-snap-align:start]",
    "tablet:box-border",
    "tablet:overflow-hidden",
    "tablet:bg-[linear-gradient(180deg,rgba(13,31,61,0.55)_0%,rgba(8,14,30,0.55)_100%)]",
    "tablet:border-[rgba(42,80,133,0.6)]",
    "tablet:hover:border-[rgba(255,206,99,0.35)]",
    "tablet:data-[selected=true]:bg-[linear-gradient(135deg,rgba(45,80,130,0.85)_0%,rgba(20,45,80,0.85)_100%)]",
    "tablet:data-[selected=true]:border-[color:var(--race-color,#ffce63)]",
    "tablet:data-[selected=true]:shadow-[0_0_10px_var(--race-color-soft,rgba(255,206,99,0.3))]",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }

// In the mobile carousel, cards of every category share one strip; when search is
// inactive only the active category's cards show. Each card hides itself when the
// container's active category is one of the OTHER three. Keyed off the `.unit-list`
// group's data attributes.
const HERO_FILTER: &str = "mobile:group-[[data-search-active=false][data-active-category=soldier]]:hidden mobile:group-[[data-search-active=false][data-active-category=worker]]:hidden mobile:group-[[data-search-active=false][data-active-category=building]]:hidden tablet:group-[[data-search-active=false][data-active-category=soldier]]:hidden tablet:group-[[data-search-active=false][data-active-category=worker]]:hidden tablet:group-[[data-search-active=false][data-active-category=building]]:hidden";
const SOLDIER_FILTER: &str = "mobile:group-[[data-search-active=false][data-active-category=hero]]:hidden mobile:group-[[data-search-active=false][data-active-category=worker]]:hidden mobile:group-[[data-search-active=false][data-active-category=building]]:hidden tablet:group-[[data-search-active=false][data-active-category=hero]]:hidden tablet:group-[[data-search-active=false][data-active-category=worker]]:hidden tablet:group-[[data-search-active=false][data-active-category=building]]:hidden";
const WORKER_FILTER: &str = "mobile:group-[[data-search-active=false][data-active-category=hero]]:hidden mobile:group-[[data-search-active=false][data-active-category=soldier]]:hidden mobile:group-[[data-search-active=false][data-active-category=building]]:hidden tablet:group-[[data-search-active=false][data-active-category=hero]]:hidden tablet:group-[[data-search-active=false][data-active-category=soldier]]:hidden tablet:group-[[data-search-active=false][data-active-category=building]]:hidden";
const BUILDING_FILTER: &str = "mobile:group-[[data-search-active=false][data-active-category=hero]]:hidden mobile:group-[[data-search-active=false][data-active-category=soldier]]:hidden mobile:group-[[data-search-active=false][data-active-category=worker]]:hidden tablet:group-[[data-search-active=false][data-active-category=hero]]:hidden tablet:group-[[data-search-active=false][data-active-category=soldier]]:hidden tablet:group-[[data-search-active=false][data-active-category=worker]]:hidden";

/// The mobile carousel hide-when-other-category filter classes for a card of `kind`.
pub(super) fn filter_class(kind: UnitKind) -> &'static str {
    match kind {
        UnitKind::Hero => HERO_FILTER,
        UnitKind::Soldier => SOLDIER_FILTER,
        UnitKind::Worker => WORKER_FILTER,
        UnitKind::Building => BUILDING_FILTER,
    }
}
