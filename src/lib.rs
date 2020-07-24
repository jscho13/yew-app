#![recursion_limit = "256"]

use yew::prelude::*;
use wasm_bindgen::prelude::*;
use anyhow::Error;
use serde_derive::{Deserialize};
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};

pub struct Model {
    link: ComponentLink<Model>,
    fetching: bool,
    data: Option<String>,
    ft: Option<FetchTask>,
    stockTicker: String,
}

#[derive(Deserialize, Debug)]
pub struct DataFromFile {
    latestPrice: f32,
    companyName: String,
    primaryExchange: String,
}

pub enum Msg {
    Update(String),
    FetchData,
    FetchReady(Result<DataFromFile, Error>),
    FetchFailed,
    Ignore,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
            fetching: false,
            data: None,
            ft: None,
            stockTicker: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update(val) => {
                log::info!("logging: {:?}", val);
                self.stockTicker = val;
            }
            Msg::FetchData => {
                self.fetching = true;

                let callback = self.link.callback(
                    move |response: Response<Json<Result<DataFromFile, Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        if meta.status.is_success() {
                            Msg::FetchReady(data)
                        } else {
                            Msg::FetchFailed
                        }
                    },
                );
                let baseUrl = format!("https://cloud.iexapis.com/stable/stock/{}/quote?token=pk_73b450b6f57349999e83bd3ff4d024d6", self.stockTicker);
                let request = Request::get(baseUrl).body(Nothing).unwrap();
                let task = FetchService::fetch(request, callback).unwrap();
                self.ft = Some(task);
            }
            Msg::FetchReady(response) => {
                self.fetching = false;
                match response {
                    Ok(v) => self.data = Some(v.latestPrice.to_string()),
                    Err(e) => self.data = Some(e.to_string()),
                }
            }
            Msg::FetchFailed => {
                self.fetching = false;
                self.data = Some(String::from("Couldn't fetch data, try again"));
            }
            Msg::Ignore => {
                return false;
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="container">
                <p>{ "Ticker Symbol" }</p>
                <input type="text"
                    value=&self.stockTicker
                    oninput=self.link.callback(|e: InputData| Msg::Update(e.value))
                />
                <button onclick=self.link.callback(|_| Msg::FetchData)>
                    { "Fetch Data" }
                </button>
                { self.view_data() }
            </div>
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }
}

impl Model {
   fn view_data(&self) -> Html {
        if let Some(latestPrice) = &self.data {
            html! {
                <>
                    <p>{ latestPrice }</p>
                </>
            }
        } else {
            html! {
                <>
                    <p>{ "Data hasn't fetched yet." }</p>
                </>
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    App::<Model>::new().mount_to_body();
}
