pub mod models;
pub mod schema;
use diesel::{ prelude::*};
use dotenv::dotenv;
use std::env;
use self::models::{URL, NewUrl};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_url(conn: &mut SqliteConnection, url: &str, short_code: &str) -> URL {
    use crate::schema::database_url;

    let new_post = NewUrl { url, short_code };

    diesel::insert_into(database_url::table)
        .values(&new_post)
        .returning(URL::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}


pub fn list_url(connection: &mut SqliteConnection)-> Vec<URL>{
    use crate::schema::database_url::dsl::*;
    
    let result= database_url
    .limit(5)
    .select(URL::as_select())
    .load(connection)
    .expect("Error loading posts");
    println!("Result:{:?}", result);
    result
}   

pub fn get_url_by_id(connection: &mut SqliteConnection, code : &str)-> URL{
    use crate::schema::database_url::dsl::*;
    
    let result= database_url
    .filter(short_code.eq(code))
    .first::<URL>(connection)
    .expect("Error loading posts");
    println!("Result:{:?}", result);
    result
} 