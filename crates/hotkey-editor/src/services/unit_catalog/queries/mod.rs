pub mod unit_filter_query;

#[cfg(test)]
pub(crate) fn assert_query<TheQuery>()
where
    TheQuery: ddd::Query,
{
}
