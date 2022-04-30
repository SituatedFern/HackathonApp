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
                            <h2>{"About Us"}</h2>
                            <a href="#">{"Blog"}</a>
                            <a href="#">{"Demo"}</a>
                            <a href="#">{"Customers"}</a>
                            <a href="#">{"Investors"}</a>
                            <a href="#">{"Terms of Service"}</a>
                        </div>
                        <div class="footer-heading footer-2">
                            <h2>{"Contact Us"}</h2>
                            <a href="#">{"Jobs"}</a>
                            <a href="#">{"Support"}</a>
                            <a href="#">{"Contact"}</a>
                            <a href="#">{"Sponsorships"}</a>
                        </div>
                        <div class="footer-heading footer-3">
                            <h2>{"Social Media"}</h2>
                            <a href="#">{"Instagram"}</a>
                            <a href="#">{"Facebook"}</a>
                            <a href="#">{"Youtube"}</a>
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
