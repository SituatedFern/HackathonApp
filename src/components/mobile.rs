use yew::prelude::*;

pub enum Msg {
}

pub struct Mobile {
}

impl Component for Mobile {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {  }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {

        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        html! {
            <div class="dropdown">
                <a href="#hero">{"Home"}</a>
                <a href="#about">{"About"}</a>
                <a href="#team">{"Our Team"}</a>
                <a href="#demos">{"Sample Demos"}</a>
            </div>
        }
    }
}
