pub fn wherein2<T>(query: &mut scylla::query::Query, sub: &[T])
where
    T: ToString,
{
    if sub.is_empty() {
        return;
    }
    query.contents.push_str(" in (");
    for v in sub {
        query.contents.push_str(v.to_string().as_str());
        query.contents.push_str(",");
    }
    query.contents.pop();
    query.contents.push_str(" )");
}