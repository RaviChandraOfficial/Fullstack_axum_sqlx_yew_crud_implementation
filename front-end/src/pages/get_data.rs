
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yewdux::functional::use_store;
use crate::api::{get_tasks, Note};
use crate::store;
use crate::router::Route;
use yew_router::prelude::Link;

#[function_component(Getdata)]
pub fn get_data() -> Html {
    let (store, _store_dispatch) = use_store::<store::Store>();

    let success_message = use_state(|| None::<Vec<Note>>);  // Change to Vec<Note>
    let error_message = use_state(|| None::<String>);

    let fetch_tasks = {

        let success_message = success_message.clone();
        let error_message = error_message.clone();
        let store = store.clone();  // Clone the store context
        Callback::from(move |_| {

            let success_message = success_message.clone();
            let error_message = error_message.clone();
            let token = store.token.clone(); // Clone the token inside the closure
            spawn_local(async move {
                match get_tasks(&token).await {
                    Ok(response) => {
                        success_message.set(Some(response.notes));  // Store the notes directly
                    },
                    Err(e) => {
                        error_message.set(Some(format!("Failed to load tasks: {:?}", e)));
                    }
                }
            });
        })
    };

    use_effect_with_deps(
        move |_| {
            fetch_tasks.emit(());
            || ()
        },
        (),
    );

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

            <div>
                <h1>{"Get Data:"}</h1>
                {
                    if let Some(notes) = (*success_message).as_ref() {
                        html! {
                            <table class={"tasks-table"} style="border-spacing: 10px; border-collapse: separate;">
                                <tr>
                                    <th style="padding: 8px;">{"ID"}</th>
                                    <th style="padding: 8px;">{"Name"}</th>
                                    <th style="padding: 8px;">{"Location"}</th>
                                    <th style="padding: 8px;">{"Data"}</th>
                                    <th Style="padding: 8px;">{"Username"}</th>
                                </tr>
                                { for notes.iter().map(|note| html! {
                                    <tr>
                                        <td style="padding: 8px; border: 1px solid gray;">{note.id.to_string()}</td>
                                        <td style="padding: 8px; border: 1px solid gray;">{&note.name}</td>
                                        <td style="padding: 8px; border: 1px solid gray;">{&note.location}</td>
                                        <td style="padding: 8px; border: 1px solid gray;">{&note.data}</td>
                                        <td style="padding: 8px; border: 1px solid gray;">{&note.user_name}</td>
                                    </tr>
                                })}
                            </table>
                        }
                    } else if let Some(err) = (*error_message).as_ref() {
                        html! { <div class={"error"}>{ err }</div> }
                    } else {
                        html! { <div>{"Data loading.........."}</div> }
                    }
                }
            </div>
        </>
    }
}
