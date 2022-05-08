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
        let mut hider = vec!["hidden"];
        if self.count == false {
            classes.push("is-active");
            hider.clear();
            hider.push("dropdown");
        }

        html! {
            <>
                <nav>
                <div class="container">
                <h1>{"Tempus"}</h1>

                <div class="menu">
                //add class="is-active" v
                <a href="#hero">{"Home"}</a>
                <a href="#about">{"About"}</a>
                <a href="#demos">{"Sample Demos"}</a>
                <a href="#team">{"Our Team"}</a>
                </div>

                <button onclick={link.callback(|_| Msg::Inverse)} class={classes}>
                <span></span>
                <span></span>
                <span></span>
                </button>
                //<p>{ self.count }</p>
                </div>
                </nav>
                <div class={hider}>
                    <a href="#hero">{"Home"}</a>
                    <a href="#about">{"About"}</a>
                    <a href="#team">{"Our Team"}</a>
                    <a href="#demos">{"Sample Demos"}</a>
                </div>
            </>
        }
    }
}
