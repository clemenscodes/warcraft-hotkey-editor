use tw_macro::tw;
// The override panel header: name/id on the left, the hotkey slot pinned right, with
// a hairline divider under it. A fixed two-column grid on the mobile panel.

classes! {
    base: tw![
        "grid",
        "grid-cols-[minmax(0,1fr)_auto]",
        "items-center",
        "gap-x-6",
        "pb-3.5",
        "border-b",
        "border-warcraft-blue-deep",
        "text-left",
    ],
    mobile: tw![
        "mobile:grid-cols-[1fr_auto]",
        "mobile:gap-2",
        "mobile:pt-0",
        "mobile:pr-0",
        "mobile:pb-2.5",
        "mobile:pl-0",
        "mobile:flex-none",
        "mobile:overflow-hidden",
        "mobile:w-full",
    ],
    tablet: tw![
        "tablet:grid-cols-[1fr_auto]",
        "tablet:gap-2",
        "tablet:pt-0",
        "tablet:pr-0",
        "tablet:pb-2.5",
        "tablet:pl-0",
        "tablet:flex-none",
        "tablet:overflow-hidden",
        "tablet:w-full",
    ],
}
