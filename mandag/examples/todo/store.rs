use std::{collections::HashMap, sync::Arc};

use mandag::{async_trait, Error, Extension, ExtensionCtx};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Todo {
    pub id: usize,
    pub name: String,
    pub completed: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateTodo {
    pub name: String,
}

#[derive(Debug, Default)]
pub struct TodosInner {
    todos: HashMap<usize, Todo>,
    current_id: usize,
}

#[derive(Debug, Default, Clone)]
pub struct Todos(Arc<RwLock<TodosInner>>);

impl Todos {
    pub fn insert(&self, todo: CreateTodo) -> Todo {
        let mut todos = self.0.write();
        todos.current_id += 1;
        let id = todos.current_id;
        let todo = Todo {
            id,
            name: todo.name,
            completed: false,
        };
        todos.todos.insert(id, todo.clone());

        todo
    }

    pub fn list(&self) -> Vec<Todo> {
        self.0.read().todos.values().cloned().collect()
    }
}

pub struct TodoStore;

#[async_trait]
impl<C: ExtensionCtx> Extension<C> for TodoStore {
    async fn build(&self, ctx: &mut C) -> Result<(), Error> {
        ctx.register(Todos::default());
        Ok(())
    }
}
