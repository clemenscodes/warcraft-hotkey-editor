use tw_macro::tw;
classes! {
    base: tw![
        "w-full",
        "flex-1",
        "min-h-[20rem]",
        "px-8",
        "py-6",
        "rounded-md",
        "border",
        "border-warcraft-blue",
        "bg-warcraft-bg-base/85",
        "text-warcraft-text-primary",
        "text-[1.8rem]/[1.45]",
        "whitespace-pre",
        "overflow-auto",
        "resize-y",
        "focus:outline-none",
        "focus:border-warcraft-gold",
        "focus:shadow-[0_0_8px_color-mix(in_oklab,var(--color-warcraft-gold)_40%,transparent)]",
    ],
    mobile: tw!["mobile:text-[1.4rem]/[1.45]"],
}
