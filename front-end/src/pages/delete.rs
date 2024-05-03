use reqwest::Client;
use serde::{Serialize, Deserialize};
use yew::prelude::*;
use yewdux::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::store;
use web_sys::HtmlInputElement;
use crate::router::Route;
use yew_router::prelude::Link;

#[function_component(DeleteData)]
pub fn delete_data() -> Html {
    let (store, _store_dispatch) = use_store::<store::Store>();
    let id = use_state(|| 0);
    let delete_result = use_state(|| None::<String>);

    let delete_data = {
        let id = id.clone();
        let delete_result = delete_result.clone();
        let store = store.clone(); 
        Callback::from(move |_| {
            let note  = Note{
                id:(*id).clone(),
            };
            // let id = id.get(); // Get the actual value from the UseStateHandle
            let token = store.token.clone(); 
            let delete_result = delete_result.clone();
            spawn_local(async move {
                let client = Client::new();
                let res = client.delete(&format!("http://localhost:3000/delete/user"))
                    .header("AUTHORIZATION", token)
                    .json(&note)
                    .send()
                    .await;
                match res {
                    Ok(_response) => {
                        delete_result.set(Some("Data deleted successfully!".to_string()));
                    }
                    Err(e) => {
                        delete_result.set(Some(format!("Error deleting data: {:?}", e)));
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
            <h1>{"Delete Data"}</h1>
            <div>
                <input type="number" placeholder="ID"
                    onchange={Callback::from(move |e: Event| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        id.set(input.value_as_number() as i32);
                    })}
                />
            </div>
            <div>
                <button onclick={delete_data}>{"Delete Data"}</button>
            </div>
            <div>
                { if let Some(result) = (*delete_result).as_ref() {
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
}