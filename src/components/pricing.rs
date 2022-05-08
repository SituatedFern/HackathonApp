use yew::prelude::*;

pub enum Msg {
}

pub struct Pricing {
}

impl Component for Pricing {
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
                <section id="pricing">
                    <div class="container7">
                        <div class="info">
                            <h1>{"Pricing"}</h1>
                            <div class="cont">
                                <div class="pricecard">
                                    <p class="bubble">{"Consumers"}</p>
                                    <h2>{"$20.99 CAD"}</h2>
                                    <h3>{"per month"}</h3>
                                    <ul>
                                        <li>{"Recreation"}</li>
                                        <li>{"Stress Relief"}</li>
                                        <li>{"Self Improvement"}</li>
                                    </ul>
                                    <p class="bubble zerodis clicker">{"Get Started"}</p>
                                </div>

                                <div class="pricecard">
                                    <p class="bubble">{"Developers"}</p>
                                    <h2>{"Free"}</h2>
                                    <h3>{"per month"}</h3>
                                    <ul>
                                        <li>{"Deployment ready core NLP model"}</li>
                                        <li>{"Non-commercial Licence"}</li>
                                    </ul>
                                    <p class="bubble zerodis clicker">{"Get Started"}</p>
                                </div>

                                <div class="pricecard">
                                    <p class="bubble">{"Enterprise"}</p>
                                    <h2>{"$99.99 CAD"}</h2>
                                    <h3>{"per month"}</h3>
                                    <ul>
                                        <li>{"Full commercial licence"}</li>
                                        <li>{"Power your product using our model"}</li>
                                    </ul>
                                    <p class="bubble zerodis clicker">{"Get Started"}</p>
                                </div>

                            </div>
                        </div>
                    </div>
                </section>
            </div>
        }
    }
}
