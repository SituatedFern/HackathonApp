use yew::prelude::*;

pub enum Msg {
}

pub struct Footer {
}

impl Component for Footer {
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
                <div class="footer-container">
                    <div class="footer">
                        <div class="footer-heading footer-1">
                            <h2>{"View Again"}</h2>
                            <a href="#hero">{"Home"}</a>
                            <a href="#about">{"About"}</a>
                            <a href="#team">{"Our Team"}</a>
                            <a href="#demos">{"Sample Demos"}</a>
                        </div>
                        <div class="footer-heading footer-2">
                            <h2>{"Content"}</h2>
                            <a href="https://youtu.be/gSH0kYG0yFc">{"Video"}</a>
                            <a href="#">{"Onepager"}</a>
                            <a href="https://medium.com/@kabirbedi1005/tempus-ai-powered-hypnotism-c6115a93d610">{"Article"}</a>
                            <a href="#">{"Presentation"}</a>
                        </div>
                        <div class="footer-heading footer-3">
                            <h2>{"Social Media"}</h2>
                            <a href="https://youtu.be/gSH0kYG0yFc">{"Youtube"}</a>
                            <a href="#">{"Odysee"}</a>
                            <a href="#">{"LinkedIn"}</a>
                            <a href="#">{"Twitter"}</a>
                        </div>
                        <div class="footer-email-form">
                            <h2>{"Join our newsletter"}</h2>
                            <input type="email" placeholder="Enter your email address"/>
                            <input type="submit" value={"Sign Up"} id="footer-email-btn"/>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
