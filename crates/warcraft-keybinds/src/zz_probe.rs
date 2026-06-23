#[cfg(test)]
mod zz_probe {
    use warcraft_database::{WARCRAFT_DATABASE, ObjectLookup};
    use warcraft_api::WarcraftObjectMeta;

    #[test]
    fn probe_offstate() {
        let units = ["egol", "htow", "hkee", "hcas"];
        for u in units {
            let Some(uo) = WARCRAFT_DATABASE.by_id(u) else { continue };
            let WarcraftObjectMeta::Unit(um) = uo.meta() else { continue };
            println!("UNIT {u} kind={:?}", um.unit_kind());
            for ab in um.abilities() {
                let aid = ab.value();
                let Some(ao) = WARCRAFT_DATABASE.by_id(aid) else { continue };
                let on = ao.default_button_position();
                let off = match ao.meta() {
                    WarcraftObjectMeta::Ability(m) => m.off_button_position(),
                    _ => None,
                };
                let alt = ao.un_tip().is_some() || ao.un_ubertip().is_some();
                if off.is_some() && off != on {
                    println!("  {aid}: on={on:?} off={off:?} alt_state={alt} (TRIGGERS BLOCK)");
                } else {
                    println!("  {aid}: on={on:?} off={off:?} alt_state={alt}");
                }
            }
        }
    }
}
