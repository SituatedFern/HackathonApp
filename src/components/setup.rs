use yew::prelude::*;

pub enum Msg {
}

pub struct Setup {
}

impl Component for Setup {
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
                <section id="setup">
                    <div class="container6">
                        <div class="info">
                            <div class="steps">
                                <h1>{"Three Easy Steps"}</h1>
                                <h2>{"1. Get Set Up"}</h2>
                                <h2>{"2. Create an Account & Log In"}</h2>
                                <h2>{"3. Start Your Session"}</h2>
                            </div>
                            <div class="boxi">
                                <iframe class="vid" src="https://www.youtube.com/embed/lQrHAhhtuDk"/>
                            </div>
                        </div>
                    </div>
                </section>
            </div>
        }
    }
}
