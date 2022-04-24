use yew::prelude::*;

pub enum Msg {
    Inverse,
}

pub struct Navbar {
    count: bool,
}

impl Component for Navbar {
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
                <nav>
                <div class="container">
                <h1>{"Cmpny Name"}</h1>

                <div class="menu">
                //add class="is-active" v
                <a href="#">{"Home"}</a>
                <a href="#">{"About"}</a>
                <a href="#">{"Projects"}</a>
                <a href="#">{"Contact"}</a>
                </div>

                <button onclick={link.callback(|_| Msg::Inverse)} class={classes}>
                <span></span>
                <span></span>
                <span></span>
                </button>
                //<p>{ self.count }</p>
                </div>
                </nav>
            </>
        }
    }
}