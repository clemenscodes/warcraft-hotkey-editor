pub mod collision_summary_query;
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
