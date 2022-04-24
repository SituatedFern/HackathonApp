use yew::prelude::*;

pub enum Msg {
    Inverse,
}

pub struct Hero {
    count: bool,
}

impl Component for Hero {
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
