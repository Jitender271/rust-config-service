use crate::schema::config;

#[derive(Queryable)]
pub struct Pair {
    pub name: String,
    pub value: String,
}

#[derive(Insertable)]
#[table_name = "config"]
pub struct NewPair<'a> {
    pub name: &'a str,
    pub value: &'a str,
}