use yew::prelude::*;

pub enum Msg {
}

pub struct About {
}

impl Component for About {
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
                <section id="about">
                    <div class="container3">
                        <div class="info">
                            <h1>{"About"}</h1>
                            <div class="moment"><p>{"Gif/Video Demo"}</p></div>
                            <p>{"An ML model learns about the preferences and personality of the subject, and adapts it dynamically depending on their reactions while in trance. This provides the foundation for a software ecosystem that can fulfill the aforementioned applications, while being scalable and cheap to deploy due to being fully digitized."}</p>
                        </div>
                    </div>
                </section>
            </div>
        }
    }
}
