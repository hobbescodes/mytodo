use super::schema::task;

#[derive(Insertable)]
#[table_name = "task"]
pub struct NewTask<'a> {
    pub title: &'a str,
}

// NOTE: you can't just derive one struct from both Queryable and Insertable.
// This would require us to set the id when we perform the insert, and we want
// to let the database engine automatically assign the id
#[derive(Queryable, Serialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
}
