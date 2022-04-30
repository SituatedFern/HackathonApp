use yew::prelude::*;

pub enum Msg {
}

pub struct Demos {
}

impl Component for Demos {
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
            <div class="bruh">
                <section id="demos">
                    <div class="container5">
                        <div class="info">
                            <h1>{"Sample Demos"}</h1>
                            <p>{"Any good samples that we have will go here. This will be a hub for the cool responses that we get from GPT-3. Images will be posted of curated responses that show what we are trying to accomplish (a proof of concept)."}</p>
                        </div>
                    </div>
                </section>
            </div>
        }
    }
}
