use yew::prelude::*;

enum Msg {
    AddOne,
}

struct CounterComponent {
    count: i64,
}

impl Component for CounterComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.count += 1;
                true //re-render component
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <>
                <nav>
                    <div class="container">
                        <h1>{"Cmpny Name"}</h1>

                        <div class="menu">
                            //add class="is-active" v
                            <a href="#">{"Home"}</a>
                            <a href="#">{"About"}</a>
                            <a href="#">{"Projects"}</a>
                            <a href="#">{"Contact"}</a>
                        </div>

                        <button onclick={link.callback(|_| Msg::AddOne)} class="hamburger">
                            <span></span>
                            <span></span>
                            <span></span>
                        </button>
                        //<p>{ self.count }</p>
                    </div>
                </nav>
            </>
        }
    }
}

fn main() {
    yew::start_app::<CounterComponent>();
}
