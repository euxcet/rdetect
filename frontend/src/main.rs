mod component;
mod route;
mod page;
mod util;

use crate::component::header::HeaderComponent;
use crate::page::dashboard::Dashboard;
use crate::page::create_task::CreateTaskPage;
use crate::page::manage_task::ManageTaskPage;
use crate::route::*;

use yew::prelude::*;

enum Msg {
    RouteUpdated(Route),
}

struct App {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
    router_agent: RouteAgentBridge,
    route_service: RouteService,
}

fn switch(switch: AppRoute) -> Html {
    match switch {
        AppRoute::Dashboard => html! {
            <Dashboard />
        },
        AppRoute::CreateTask => html! {
            <CreateTaskPage />
        },
        AppRoute::ManageTask(name) => html! {
            <ManageTaskPage name={name} />
        },
        AppRoute::NotFound(_) => html! {
        },
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let route_service = RouteService::new();
        on_route_change(route_service.get_route(), true);
        let router_agent = RouteAgentBridge::new(link.callback(Msg::RouteUpdated));
        Self {
            link,
            value: 0,
            router_agent,
            route_service,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RouteUpdated(r) => {
                on_route_change(r, true);
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <HeaderComponent />
                <Router render = Router::render(switch) />
                /*
                <Router
                    render = Router::render(move |switch: AppRoute| {
                        match switch {
                            AppRoute::Dashboard => html! {
                                <Dashboard />
                            },
                            AppRoute::NotFound(_) => html! {
                            },
                        }
                    })
                />
                */
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}