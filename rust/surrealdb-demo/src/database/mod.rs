use surrealdb::Session;

pub struct SurrealDbClient {
    session: Session,
    password: String,
    username: String,
    url: String,
}

impl SurrealDbClient {
    pub fn new<S>(url: &str, namespace: &str, password: S, username: S) -> Self
    where
        S: Into<String> + Clone,
    {
        let session = Session::default().with_db(url).with_ns(namespace);
    }
}
