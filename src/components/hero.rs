use yew::prelude::*;

pub enum Msg {
}

pub struct Hero {
}

impl Component for Hero {
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
                <section id="hero">
                    <div class="container2">
                        <div class="info">
                            <h1>{"This is Tempus"}</h1>
                            <h2>{"AI Powered Hypnosis"}</h2>
                            <p>{"Hypnosis has been mostly restricted by the fact that it relies on human interactions. We can simulate these interactions using Natural Language Processing and Machine Learning to create fully tailored hypnotic experiences. Our model will be able to adapt to the needs and goals of the subject and could be applied to a large demographic."}</p>
                            <a href="#">{"Whitepaper"}</a>
                        </div>
                    </div>
                </section>
            </div>
        }
    }
}
