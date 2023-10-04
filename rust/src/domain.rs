mod services;
mod todolist;
mod use_cases;

pub use services::TodoListStore;
pub use todolist::Event as TodoListEvent;
pub use todolist::TodoList;

pub struct Domain {
    pub todolist: Box<dyn TodoListStore>,
}
