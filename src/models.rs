use crate::schema::database_url;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Debug,Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::database_url)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct URL {
    pub id: i32,
    pub url: String,
    pub short_code: String,
}

#[derive(Insertable)]
#[diesel(table_name = database_url)]
pub struct NewUrl<'a> {
    pub url: &'a str,
    pub short_code: &'a str,
}