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
                                    <p class="bubble">{"1 Free Session"}</p>
                                    <h2>{"$11.99 CAD"}</h2>
                                    <h3>{"per month"}</h3>
                                    <ul>
                                        <li>{"Normal 5 min Sessions"}</li>
                                        <li>{"Something else"}</li>
                                        <li>{"Something else"}</li>
                                    </ul>
                                    <p class="bubble zerodis">{"Get Started"}</p>
                                </div>

                                <div class="pricecard">
                                    <p class="bubble">{"1 Free Session"}</p>
                                    <h2>{"$16.99 CAD"}</h2>
                                    <h3>{"per month"}</h3>
                                    <ul>
                                        <li>{"Thorough 10 min Sessions"}</li>
                                        <li>{"Something else"}</li>
                                        <li>{"Something else"}</li>
                                    </ul>
                                    <p class="bubble zerodis">{"Get Started"}</p>
                                </div>

                                <div class="pricecard">
                                    <p class="bubble">{"1 Free Session"}</p>
                                    <h2>{"$20.99 CAD"}</h2>
                                    <h3>{"per month"}</h3>
                                    <ul>
                                        <li>{"Thorough 20 min Sessions"}</li>
                                        <li>{"Something else"}</li>
                                        <li>{"Something else"}</li>
                                    </ul>
                                    <p class="bubble zerodis">{"Get Started"}</p>
                                </div>

                            </div>
                        </div>
                    </div>
                </section>
            </div>
        }
    }
}
