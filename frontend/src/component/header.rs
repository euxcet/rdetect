use yew::prelude::*;

pub struct HeaderComponent {
    link: ComponentLink<Self>,
}

pub enum Msg {

}

impl Component for HeaderComponent {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <nav class="navbar navbar-light">
                <div class="container-fluid">
                    <ul class="nav">
                        <li class="nav-item">
                            <div class="navbar-brand">
                                <p>{"Detect"}</p>
                            </div>
                        </li>
                    </ul>
                </div>
            </nav>
        }
    }
}