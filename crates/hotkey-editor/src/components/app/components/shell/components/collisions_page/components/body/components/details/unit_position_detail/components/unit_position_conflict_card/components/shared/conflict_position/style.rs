use tw_macro::tw;
classes! {
    base: tw![
        "self-start",
        "h-[72px]",
        "inline-flex",
        "items-center",
        "justify-center",
        "[&>*]:h-[60px]",
        "[&>*]:w-[calc(60px/3*4)]",
        "data-[top=true]:self-center",
        "data-[top=true]:h-auto",
        "data-[top=true]:mb-2.5",
    ],
}
