use tw_macro::tw;
// Fills the host box (`size-full`) and centres the wordmark within it vertically
// (`items-center`), so when the header stretches the host to the full bar height the brand
// sits centred rather than pinned to the top. It lays the wordmark out as one cqi-scaled
// row: the gap between flourishes and title is a container-query length, so it shrinks in
// step with the title and flourishes as the host narrows. No fixed lengths — the host owns
// the size.

classes! {
    base: tw![
        "flex",
        "flex-row",
        "items-center",
        "justify-start",
        "size-full",
        "gap-[2.2cqi]",
        "bg-transparent",
        "border-0",
        "p-0",
        "cursor-pointer",
        "text-left",
        "transition-[filter,text-shadow]", "duration-fast",
        "hover:filter-[brightness(1.15)]",
        "focus:outline-none",
        "focus-visible:[outline:2px_solid_var(--color-warcraft-highlight)]",
        "focus-visible:outline-offset-2",
    ],
}
