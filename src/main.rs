mod components;

use components::about::About;
use components::hero::Hero;
use components::navbar::Navbar;
use components::team::Team;
use components::demos::Demos;
use components::footer::Footer;
use components::setup::Setup;
use components::pricing::Pricing;

use yew::prelude::*;

enum Msg {
}

struct App {
}

impl Component for App {
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
            <>
                <Navbar/>
                <Hero/>
                <About/>
                <Setup/>
                <Demos/>
                <Team/>
                <Pricing/>
                <Footer/>
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
