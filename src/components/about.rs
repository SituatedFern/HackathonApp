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
                            <p>{"Lorem ipsum, dolor sit amet consectetur adipisicing elit. Voluptatem vero ratione vitae dicta explicabo
        perferendis amet quis provident molestiae magni!"}</p>
                        </div>
                    </div>
                </section>
            </div>
        }
    }
}
