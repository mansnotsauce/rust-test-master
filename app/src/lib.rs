use seed::prelude::*;
use seed::*;
use seed::browser::fetch as fetch;
use serde::Serialize;

#[derive(Default)]
struct Model {
    items: Vec<String>,
    new_item: String,
    error: Option<String>,
}

#[derive(Serialize)]
struct Item {
    item: String
}

#[derive(Serialize)]
struct Items {
    items: Vec<String>
}

enum Msg {
    FetchedItems(fetch::Result<Vec<String>>),
    NewItemInputChanged(String),
    Save,
    Clear,
    Delete(String)
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    use Msg::*;

    match msg {
        FetchedItems(resp) => match resp {
            Ok(items) => model.items = items,
            Err(e) => model.error = Some(format!("{:?}", e)),
        },
        NewItemInputChanged(new_item) => model.new_item = new_item,
        Save => {
            if !model.new_item.trim().is_empty() {
                _orders.skip().perform_cmd({
                    let input = model.new_item.clone().trim().to_string();
                    async { FetchedItems(save_item(input).await) }
                });
                model.new_item = "".to_string();
            }
        },
        Clear => {
            if model.items.len() != 0 {
                _orders.skip().perform_cmd(async { FetchedItems(clear_items().await) });
            }
        },
        Delete(item) => {
            _orders.skip().perform_cmd(async { FetchedItems(delete_item(item).await) });
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        input![
            attrs! {
                At::Value => model.new_item
            },
            input_ev(Ev::Input, Msg::NewItemInputChanged)
        ],
        div![
            button!["Save", ev(Ev::Click, |_| Msg::Save)],
            span![" "],
            button!["Clear", ev(Ev::Click, |_| Msg::Clear)]
        ],
        ul![
            model.items.iter().map(|item| {
                let item_to_delete = item.to_string();
                li![
                    div![
                        item,
                        span![" "],
                        button![
                            C!["button"],
                            i![C!["fa fa-trash"]],
                            ev(Ev::Click, |_| Msg::Delete(item_to_delete))
                        ]
                    ],
                ]
            })
        ]
    ]
}

async fn get_todo_items() -> fetch::Result<Vec<String>> {
    Request::new("/api/todo")
        .method(fetch::Method::Get)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

async fn save_item(new_item: String) -> fetch::Result<Vec<String>> {
    fetch::Request::new("/api/new")
        .method(fetch::Method::Post)
        .json(&Item { item: new_item.to_string() })?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

async fn clear_items() -> fetch::Result<Vec<String>> {
    Request::new("/api/clear")
        .method(fetch::Method::Get)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

async fn delete_item(item_to_delete: String) -> fetch::Result<Vec<String>> {
    fetch::Request::new("/api/delete")
        .method(fetch::Method::Post)
        .json(&Item { item: item_to_delete.to_string() })?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async { Msg::FetchedItems(get_todo_items().await) });
    Model::default()
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
