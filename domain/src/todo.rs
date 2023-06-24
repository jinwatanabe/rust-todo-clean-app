
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Todos(pub Vec<Todo>);

impl IntoIterator for Todos {
    type Item = Todo;
    type IntoIter = std::vec::IntoIter<Todo>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Todos {
    type Item = &'a Todo;
    type IntoIter = std::slice::Iter<'a, Todo>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Todo {
	pub id: TodoId,
	pub title: TodoTitle,
	pub done: TodoDone,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TodoId(pub i32);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TodoTitle(pub String);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TodoDone(pub bool);