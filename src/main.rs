mod components;

use components::hero::Hero;
use components::navbar::Navbar;

use yew::prelude::*;

enum Msg {
    Inverse,
}

struct App {
    count: bool,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { count: true }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Inverse => {
                self.count = !self.count;
                true //re-render component
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let mut classes = vec!["hamburger"];
        if self.count == false {
            classes.push("is-active")
        }

        html! {
            <>
                <Navbar/>
                <Hero/>
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
