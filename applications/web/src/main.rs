mod runtime;

use runtime::Runtime;
use std::rc::Rc;
use ui::app::render;

fn main() {
    let runtime = Rc::new(Runtime::new());

    render(runtime);
}
