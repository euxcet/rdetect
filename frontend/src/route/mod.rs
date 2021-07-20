use yew_router::prelude::*;
use yew_router::agent::RouteRequest;
use yew_router::switch::Permissive;

#[derive(Switch, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[to = "/!"]
    Dashboard,
    #[to = "/{}"]
    NotFound(Permissive<String>),
}

impl AppRoute {
    pub fn requires_auth(&self) -> bool {
        match &self {
            AppRoute::Dashboard => false,
            _ => true,
        }
    }

    pub fn redirect_if_logged_in(&self) -> bool {
        match &self {
            AppRoute::Dashboard => false,
            _ => false,
        }
    }
}

pub fn on_route_change(new_route: yew_router::route::Route, is_authenticated: bool) {
    match AppRoute::switch(new_route.clone()) {
        None => {}
        Some(a) => {
            let mut dispatch = RouteAgentDispatcher::new();
            if a.requires_auth() && !is_authenticated {
            } else if a.redirect_if_logged_in() && is_authenticated {
                let redirect = Route::from(AppRoute::Dashboard);
                dispatch.send(RouteRequest::ReplaceRoute(redirect))
            }
        }
    }
}

#[allow(dead_code)]
pub type Route = yew_router::route::Route;

#[allow(dead_code)]
pub type RouteService = yew_router::service::RouteService;

#[allow(dead_code)]
pub type RouteAgent = yew_router::agent::RouteAgent;

#[allow(dead_code)]
pub type RouteAgentBridge = yew_router::agent::RouteAgentBridge;

#[allow(dead_code)]
pub type RouteAgentDispatcher = yew_router::agent::RouteAgentDispatcher;

#[allow(dead_code)]
pub type RouterAnchor = yew_router::components::RouterAnchor<AppRoute>;

#[allow(dead_code)]
pub type RouterButton = yew_router::components::RouterButton<AppRoute>;

#[allow(dead_code)]
pub type Router = yew_router::router::Router<AppRoute>;