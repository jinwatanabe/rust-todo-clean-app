use serde::Deserialize;

diesel::table! {
    todos (id) {
        id -> Integer,
        title -> Varchar,
        done -> Bool,
    }
}


#[derive(Deserialize, Debug, Queryable)]
pub struct Todo {
	pub id: i32,
	pub title: String,
	pub done: bool,
}

#[derive(Insertable)]
#[table_name = "todos"]
pub struct NewTodo<'a> {
    pub title: &'a str,
    pub done: bool,
}