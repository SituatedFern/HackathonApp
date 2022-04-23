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

                <ul class="nav">
                    <li><a class="active" ><i>{"Home"}</i></a></li>
                    <li><a class="active" ><i>{"About"}</i></a></li>
                    <li><a class="active" ><i>{"Our Team"}</i></a></li>
                    <li><a class="active" ><i>{"Try it out"}</i></a></li>
                </ul>

                //<h1 class="heroname">{"Our Company Name"}</h1>



                <div class="container">
                    <p>{self.count}</p>
                    <button onclick={link.callback(|_| Msg::AddOne)}>{"+1"}</button>
                </div>
            </>
        }
    }
}

fn main() {
    yew::start_app::<CounterComponent>();
}
