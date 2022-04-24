use yew::prelude::*;

pub enum Msg {
    Inverse,
}

pub struct About {
    count: bool,
}

impl Component for About {
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
