use yew::prelude::*;

pub enum Msg {
}

pub struct Team {
}

impl Component for Team {
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
                <section id="team">
                    <div class="container4">
                        <div class="info">
                            <h1>{"Our Team"}</h1>
                            <div class="cont">
                                <div class="card">
                                    <div class="card-image car-1"></div>
                                    <h2>{"Ashwin Gnanam"}</h2>
                                    <p><a href="https://atgnanam.ca">{"atgnanam.ca"}</a></p>
                                </div>
                                <div class="card">
                                    <div class="card-image car-2"></div>
                                    <h2>{"Huraira Khan"}</h2>
                                    <p><a href="https://www.linkedin.com/in/huraira-khan-70a4a91bb/">{"@HurairaKhan"}</a></p>
                                </div>
                                <div class="card">
                                    <div class="card-image car-3"></div>
                                    <h2>{"William Kou"}</h2>
                                    <p>{"@TooKouForSchool"}</p>
                                </div>
                                <div class="card">
                                    <div class="card-image car-4"></div>
                                    <h2>{"Kabir Bedi"}</h2>
                                    <p>{"@TooKouForSchool"}</p>
                                </div>
                                <div class="card">
                                    <div class="card-image car-5"></div>
                                    <h2>{"Kiara Soin"}</h2>
                                    <p>{"@TooKouForSchool"}</p>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>
            </div>
        }
    }
}
