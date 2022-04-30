use yew::prelude::*;

pub enum Msg {
    Inverse,
}

pub struct Demos {
    count: bool,
}

impl Component for Demos {
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
