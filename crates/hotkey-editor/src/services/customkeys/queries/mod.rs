//! The application-layer queries the [`super::service::CustomKeysService`]
//! answers. Each is a named [`ddd::Query`] over the `CustomKeys` aggregate: the
//! read-side counterpart to the [`super::commands`], a first-class request to
//! observe state without changing it. A query keeps the renderer off the domain
//! crate — instead of a component calling `SystemBindingMap::build` or
//! `CrossUnitCollisionReport::compute` at render time, it asks the service, which
//! reads the live aggregate reactively and hands back a view. Queries are
//! `ApplicationLayer`; they live here in the renderer crate, not in the
//! pure-domain `warcraft-keybinds` crate.

pub mod cross_unit_collisions_query;
pub mod resolve_preview_query;
pub mod slot_binding_query;
pub mod unit_collisions_query;

#[cfg(test)]
pub(crate) fn assert_query<TheQuery>()
where
    TheQuery: ddd::Query,
{
}
