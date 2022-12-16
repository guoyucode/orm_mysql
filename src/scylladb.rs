use scylla::query::Query;




pub struct ScyllaQuery {
    pub query: Query,
}

impl From<String> for ScyllaQuery {
    fn from(s: String) -> ScyllaQuery {
        ScyllaQuery {
            query: Query::new(s)
        }
    }
}

impl<'a> From<&'a str> for ScyllaQuery {
    fn from(s: &'a str) -> ScyllaQuery {
        ScyllaQuery {
            query: Query::new(s.to_owned())
        }
    }
}

impl ScyllaQuery {

    pub fn wherein<T>(&mut self, sub: &Vec<T>) -> Query
        where T: ToString {

        if sub.is_empty() {
            return self.query.clone();
        }
        self.query.contents.push_str(" in (");
        for v in sub {
            self.query.contents.push_str(v.to_string().as_str());
            self.query.contents.push_str(",");
        }
        self.query.contents.pop();
        self.query.contents.push_str(" )");
        self.query.clone()
    }

    pub fn wherein2<T>(&mut self, sub: &[T]) -> Query
        where T: ToString {

        if sub.is_empty() {
            return self.query.clone();
        }
        self.query.contents.push_str(" in (");
        for v in sub {
            self.query.contents.push_str(v.to_string().as_str());
            self.query.contents.push_str(",");
        }
        self.query.contents.pop();
        self.query.contents.push_str(" )");
        self.query.clone()
    }
}