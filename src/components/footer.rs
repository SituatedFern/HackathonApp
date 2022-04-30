use yew::prelude::*;

pub enum Msg {
}

pub struct Footer {
}

impl Component for Footer {
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
            <>
            </>
        }
    }
}
