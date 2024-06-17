use reqwest::Client;
use serde::{Serialize, Deserialize};
use yew::prelude::*;
use yewdux::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use crate::store;
use crate::router::Route;
use yew_router::prelude::Link;

#[function_component(PostData)]
pub fn post_data() -> Html {
    let (store, _store_dispatch) = use_store::<store::Store>();
    let id = use_state(|| 0);
    let name = use_state(|| "".to_string());
    let location = use_state(|| "".to_string());
    let data = use_state(|| "".to_string());
    let post_result = use_state(|| None::<String>);

    let post_data = {
        let id = id.clone();
        let name = name.clone();
        let location = location.clone();
        let data = data.clone();
        let post_result = post_result.clone();
        let store = store.clone(); 
        Callback::from(move |_| {
            let note = Note {
                id: (*id).clone(),
                name: (*name).clone(),
                location: (*location).clone(),
                data: (*data).clone(),
            };
            let token = store.token.clone(); 
            let post_result = post_result.clone();
            spawn_local(async move {
                let client = Client::new();
                let res = client.post("http://localhost:3000/post/user")
                    .header("AUTHORIZATION", token)
                    .json(&note)
                    .send()
                    .await;
                match res {
                    Ok(_response) => {
                        post_result.set(Some("Data posted successfully!".to_string()));
                    }
                    Err(e) => {
                        post_result.set(Some(format!("Error posting data: {:?}", e)));
                    }
                }
            });
        })
    };

    html! {
        <>
        <table style="width: 100%; background: #f0f0f0; border-bottom: 1px solid black;">
        <tr>
            <td style="text-align: center; border-right: 1px solid black;">
                <Link<Route> to={Route::Getdata}>{ "GET" }</Link<Route>>
            </td>
            <td style="text-align: center; border-right: 1px solid black;">
                <Link<Route> to={Route::PostData}>{ "POST" }</Link<Route>>
            </td>
            <td style="text-align: center; border-right: 1px solid black;">
                <Link<Route> to={Route::PutData}>{ "PUT" }</Link<Route>>
            </td>
            <td style="text-align: center;">
                <Link<Route> to={Route::DeleteData}>{ "DELETE" }</Link<Route>>
            </td>
        </tr>
        </table>
            <h1>{"Post Data"}</h1>
            <div>
                <input type="number" placeholder="ID"
                    onchange={Callback::from(move |e: Event| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        id.set(input.value_as_number() as i32);
                    })}
                />
                </div>
                <div>
                <input type="text" placeholder="Name"
                    oninput={Callback::from(move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        name.set(input.value());
                    })}
                />
                </div>
                <div>
                <input type="text" placeholder="Location"
                    oninput={Callback::from(move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        location.set(input.value());
                    })}
                />
                </div>
                <div>
                <input type="text" placeholder="Data"
                    oninput={Callback::from(move |e: InputEvent| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        data.set(input.value());
                    })}
                />
                </div>
                <div>
                <button onclick={post_data}>{"Post Data"}</button>
                </div>
            <div>
                { if let Some(result) = (*post_result).as_ref() {
                    html! { <p>{ result }</p> }
                } else {
                    html! {}
                }}
            </div>
        </>
    }
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Note {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub data: String,
}
