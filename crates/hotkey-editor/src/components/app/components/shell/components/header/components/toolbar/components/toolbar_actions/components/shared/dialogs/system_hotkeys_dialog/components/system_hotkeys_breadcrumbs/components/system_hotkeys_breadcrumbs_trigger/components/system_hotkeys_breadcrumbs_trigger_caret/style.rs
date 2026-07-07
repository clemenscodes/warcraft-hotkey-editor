use tw_macro::tw;
classes! {
    base: tw!["data-[open=true]:rotate-180"],
    mobile: tw![
        "mobile:flex-none",
        "mobile:text-[0.9em]",
        "mobile:leading-none",
        "mobile:[transition:transform_0.18s_ease]",
    ],
    tablet: tw![
        "tablet:flex-none",
        "tablet:text-[0.9em]",
        "tablet:leading-none",
        "tablet:[transition:transform_0.18s_ease]",
    ],
}
