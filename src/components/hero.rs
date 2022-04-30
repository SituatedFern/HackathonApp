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
                            <h1>{"This is Heading"}</h1>
                            <h2>{"Lorem ipsum dolor sit amet."}</h2>
                            <p>{"Lorem ipsum, dolor sit amet consectetur adipisicing elit. Voluptatem vero ratione vitae dicta explicabo
        perferendis amet quis provident molestiae magni!"}</p>
                            <a href="#">{"Click Me"}</a>
                        </div>
                    </div>
                </section>
            </div>
        }
    }
}
