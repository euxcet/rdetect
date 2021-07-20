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
    query_path = "./graphql/create_task.graphql",
    response_derives = "Debug"
)]
struct CreateTask;

pub struct CreateTaskPage {
    link: ComponentLink<Self>,
    name: String,
    error: Option<String>,
}

#[derive(Debug)]
pub enum Msg {
    Create,
    Update(String),
    ReceiveResponse(Result<Vec<Value>, anyhow::Error>),
}

impl Component for CreateTaskPage {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            name: String::new(),
            error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Create => {
                let build_query = CreateTask::build_query(create_task::Variables{ 
                    task: create_task::NewTask {
                        name: self.name.clone(),
                        imageFolder: String::from("f1"),
                        xmlFolder: String::from("f2"),
                        labels: vec![String::from("1")],
                    }
                });
                let query = Json(&build_query);

                let request = Request::post(&gql_uri()).body(query).expect("Could not build request.");

                let callback = self.link.callback(
                    |response: Response<Result<String, anyhow::Error>>| {
                        let resp_body = response.into_body();
                        let resp_str = resp_body.as_ref().unwrap();

                        let tasks_value: Value =
                            from_str(&resp_str).unwrap();
                        let tasks_vec = tasks_value["data"]
                            ["createTask"]
                            .as_array()
                            .unwrap()
                            .to_owned();
                        Msg::ReceiveResponse(Ok(tasks_vec))
                    }
                );

                let _task = FetchService::fetch(request, callback).expect("failed to start request");
            }
            Msg::ReceiveResponse(data) => {
                match data {
                    Ok(_vec) => {
                    }
                    Err(error) => self.error = Some(error.to_string()),
                }
            }
            Msg::Update(val) => {
                self.name = val;
            } 
        }
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <form>
                    <div class="mb-3">
                        <label class="form-label">{"Name"}</label>
                        <input
                            class="form-text"
                            placeholder="Task name"
                            value={self.name.clone()}
                            oninput={self.link.callback(|e: InputData| Msg::Update(e.value))}
                        />
                    </div>
                    <button class="btn btn-primary" onclick=self.link.callback(|_| Msg::Create)> { "Create" } </button>
                </form>
            </>
        }
    }

}