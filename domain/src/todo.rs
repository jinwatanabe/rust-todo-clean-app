
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Todos(pub Vec<Todo>);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Todo {
	pub id: TodoId,
	pub title: TodoTitle,
	pub done: TodoDone,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TodoId(pub u32);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TodoTitle(pub String);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TodoDone(pub bool);