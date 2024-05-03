use crate::api;
use crate::components::molecules::account_form_login::{AccountForm, Action, User};
use crate::router::Route;
use crate::store::login_token;
use crate::store::Store;

use stylist::yew::styled_component;
use stylist::Style;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::*;


#[styled_component(Login)]
pub fn login() -> Html {
    let stylesheet = Style::new(css!(
        r#"
          section {
            display: flex;
            justify-content: center;
          }

          section > div {
            width: 60vw;
          }

          .message {
            color: green;
            font-size: 1em;
            text-align: center;
            margin-top: 20px;
          }

          .error {
            color: red;
            font-size: 1em;
            text-align: center;
            margin-top: 20px;
          }
        "#
    ))
    .unwrap();

    let history = use_navigator().unwrap();
    let (_store, store_dispatch) = use_store::<Store>();
    let success_message = use_state(|| None::<String>);
    let error_message = use_state(|| None::<String>);

    let onsubmit = {
        let store_dispatch = store_dispatch.clone();
        let history = history.clone();
        let success_message = success_message.clone();
        let error_message = error_message.clone();

        Callback::from(move |user: User| {
            let store_dispatch = store_dispatch.clone();
            let history = history.clone();
            let error_message = error_message.clone();
            
            spawn_local(async move {
                match api::login(user.username, user.password).await {
                    Ok(auth_response) => {
                        history.push(&Route::Home);
                        login_token(&auth_response, store_dispatch);
                    },
                    Err(e) => {
                      println!("{:?}", e);
                        let error_message_str = e.as_string().unwrap_or_else(|| "Unknown error".to_string());
                        error_message.set(Some(format!("Login failed: {}", error_message_str)));
                    }
                }
            });
        })
    };

    html! {
      <div class={stylesheet}>
        <h1>{"Login"}</h1>
        <section>
          <div>
            <AccountForm {onsubmit} action={Action::Login} />
            if let Some(message) = (*success_message).as_ref() {
                <div class={"message"}>{ message }</div>
            }
            if let Some(error) = (*error_message).as_ref() {
                <div class={"error"}>{ error }</div>
            }
          </div>
        </section>
      </div>
    }
}

