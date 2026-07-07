use tw_macro::tw;
classes! {
    base: tw![
        "w-full",
        "flex-1",
        "min-h-80",
        "px-8",
        "py-6",
        "rounded-md",
        "border",
        "border-warcraft-blue",
        "bg-warcraft-bg-base/85",
        "text-warcraft-text-primary",
        "text-2xl",
        "leading-prose",
        "whitespace-pre",
        "overflow-auto",
        "resize-y",
        "focus:outline-none",
        "focus:border-warcraft-gold",
        "focus:shadow-glow-soft",
    ],
    mobile: tw!["mobile:text-lg", "mobile:leading-prose"],
}
