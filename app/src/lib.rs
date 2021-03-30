use seed::prelude::*;
use seed::*;
use seed::browser::fetch as fetch;
use serde::Serialize;

#[derive(Default, Debug)]
struct Model {
    items: Vec<String>,
    new_item: String,
    index_to_swap: isize,
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

#[derive(Serialize)]
struct Index {
    index: usize
}

#[derive(Serialize)]
struct Indexes {
    indexes: Vec<usize>
}

enum Msg {
    FetchedItems(fetch::Result<Vec<String>>),
    NewItemInputChanged(String),
    Save,
    Clear,
    Delete(usize),
    Swap(usize)
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
        Delete(index) => {
            _orders.skip().perform_cmd(async move { FetchedItems(delete_item(index).await) });
        },
        Swap(index) => {
            let index: isize = index as isize;
            if model.index_to_swap == -1 {
                model.index_to_swap = index;
            }
            else if model.index_to_swap == index {
                model.index_to_swap = -1;
            }
            else {
                let first_index = model.index_to_swap.clone();
                model.index_to_swap = -1;
                _orders.skip().perform_cmd(async move { FetchedItems(swap_items(first_index, index).await) });
            }
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    let mut index: usize = 0;
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
                let row_index = index;
                index += 1;
                let mut class_name = "list_item";
                if model.index_to_swap == row_index as isize {
                    class_name = "list_item_selected";
                }
                li![
                    div![
                        span![
                            C![class_name],
                            item,
                            ev(Ev::Click, move |_| Msg::Swap(row_index))
                        ],
                        span![" "],
                        button![
                            C!["button"],
                            i![C!["fa fa-trash"]],
                            ev(Ev::Click, move |_| Msg::Delete(row_index))
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

async fn delete_item(index_to_delete: usize) -> fetch::Result<Vec<String>> {
    fetch::Request::new("/api/delete")
        .method(fetch::Method::Post)
        .json(&Index { index: index_to_delete })?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

async fn swap_items(first_index: isize, second_index: isize) -> fetch::Result<Vec<String>> {
    fetch::Request::new("/api/swap")
        .method(fetch::Method::Post)
        .json(&Indexes { indexes: vec![first_index as usize, second_index as usize] })?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async { Msg::FetchedItems(get_todo_items().await) });
    Model { index_to_swap: -1, ..Model::default() }
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
