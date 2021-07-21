use graphql_client::GraphQLQuery;
use serde_json::{Value, from_str};
use std::fmt::Debug;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::{spawn_local, JsFuture};

use yew::services::fetch::{Request, Response, FetchService, FetchTask};
use yew::format::Json;
use yew::prelude::*;

use crate::util::{constant::ObjectId, common::gql_uri};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/get_task.graphql",
    response_derives = "Debug"
)]
struct GetTask;

pub struct ManageTaskPage {
    props: Props,
    link: ComponentLink<Self>,
    list: Option<Value>,
    fetch_task: Option<FetchTask>,
    error: Option<String>,
}

#[derive(Debug)]
pub enum Msg {
    PassRequest,
    ReceiveResponse(Result<Value, anyhow::Error>)
}

impl ManageTaskPage {
    fn view_data(&self) -> Html {
        match self.list {
            Some(ref task) => {
                html! {
                    <div>
                        <h2> { &task["name"].as_str().unwrap() } </h2>
                        <p> { &task["imageFolder"].as_str().unwrap() } </p>
                        <p> { &task["xmlFolder"].as_str().unwrap() } </p>
                        <a href={ String::from("/Label/") + &task["name"].as_str().unwrap() }> { "Label Now" } </a>
                    </div>
                }
            }
            None => {
                html! {
                    <p> { "No tasks." } </p>
                }
            }
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub name: String,
}

impl Component for ManageTaskPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            list: None,
            fetch_task: None,
            error: None,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let link = self.link.clone();
            if first_render {
                spawn_local(async move {
                    link.send_message(Msg::PassRequest)
                })
            }
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PassRequest => {
                let build_query = GetTask::build_query(get_task::Variables { name: Some(self.props.name.clone()) });
                // let build_query = AllTasks::build_query(all_tasks::Variables {});
                let query = Json(&build_query);

                let request = Request::post(&gql_uri()).body(query).expect("Could not build request.");

                let callback = self.link.callback(
                    |response: Response<Result<String, anyhow::Error>>| {
                        let resp_body = response.into_body();
                        let resp_str = resp_body.as_ref().unwrap();

                        let tasks_value: Value =
                            from_str(&resp_str).unwrap();
                        let tasks_vec = tasks_value["data"]
                            ["getTask"]
                            .to_owned();

                        Msg::ReceiveResponse(Ok(tasks_vec))
                    }
                );

                let task = FetchService::fetch(request, callback)
                    .expect("failed to start request");

                self.fetch_task = Some(task);

                true
            }
            Msg::ReceiveResponse(data) => {
                match data {
                    Ok(tasks_vec) => {
                        self.list = Some(tasks_vec);
                    }
                    Err(error) => self.error = Some(error.to_string()),
                }
                self.fetch_task = None;

                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        let link = self.link.clone();
        spawn_local(async move {
            link.send_message(Msg::PassRequest)
        });
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                { self.view_data() }
            </>
        }
    }

}