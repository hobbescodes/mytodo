use diesel::{prelude::*, sqlite::SqliteConnection};

pub mod models;
pub mod schema;

// NOTE: Anything more serious than the application being built will need a better mechanism for
// setting the path to the database
pub fn establish_connection() -> SqliteConnection {
    let db = "./testdb.sqlite3";
    SqliteConnection::establish(db).unwrap_or_else(|_| panic!("Error connecting to {}", db))
}

pub fn create_task(connection: &SqliteConnection, title: &str) {
    let task = models::NewTask { title };

    // Takes the table from our schema, gives us back an object that we can add to with values,
    // which gives us back an object that we can execute.
    // NOTE: In a real app, should do a better job of error handling
    diesel::insert_into(schema::task::table)
        .values(&task)
        .execute(connection)
        .expect("Error inserting new task");
}

pub fn query_task(connection: &SqliteConnection) -> Vec<models::Task> {
    schema::task::table
        .load::<models::Task>(connection)
        .expect("Error loading tasks")
}
