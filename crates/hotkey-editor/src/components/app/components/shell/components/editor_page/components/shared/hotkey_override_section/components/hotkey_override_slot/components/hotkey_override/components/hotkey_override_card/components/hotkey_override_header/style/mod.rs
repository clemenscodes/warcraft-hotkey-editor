use tw_macro::tw;

classes! {
    base: tw![
        "grid",
        "grid-cols-[minmax(0,1fr)_auto]",
        "items-center",
        "gap-x-6",
        "min-h-20",
        "pb-3.5",
        "border-b",
        "border-warcraft-blue-deep",
        "text-left",
    ],
    mobile: tw![
        "mobile:grid-cols-[1fr_auto]",
        "mobile:min-h-0",
        "mobile:h-[3.4em]",
        "mobile:gap-[0.6em]",
        "mobile:p-0",
        "mobile:border-b-0",
        "mobile:flex-none",
        "mobile:overflow-hidden",
        "mobile:w-full",
    ],
    tablet: tw![
        "tablet:grid-cols-[1fr_auto]",
        "tablet:min-h-[4.6rem]",
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
