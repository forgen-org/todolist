mod runtime;

use interactions::presenters::CreateTaskForm;
use runtime::Runtime;
use std::rc::Rc;
use ui::app::{render, AppContext};

fn main() {
    let runtime = Rc::new(Runtime::new());

    let create_task_form = CreateTaskForm {
        runtime: runtime.clone(),
    };

    let app_context = AppContext {
        create_task_form: Rc::new(create_task_form),
    };

    render(app_context);
}
