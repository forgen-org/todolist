mod app;
mod components;
mod ionic;
mod runtime;

use app::App;
use runtime::Runtime;

fn main() {
    let runtime = Runtime::new();
    yew::Renderer::<App>::with_props(runtime).render();
}
