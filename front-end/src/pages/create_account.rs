use crate::api;
use crate::components::molecules::account_form::{AccountForm, Action, User};
use crate::router::Route;

use serde::{Deserialize, Serialize};
use stylist::yew::styled_component;
use stylist::Style;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::use_navigator;



#[derive(Serialize, Deserialize, Clone, PartialEq, Default)]
struct SignUpData {
    username: String,
    email: String,
    password: String,
}


#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub on_success: Callback<()>,
    pub on_error: Callback<String>,
}


#[styled_component(CreateAccount)]
pub fn create_account() -> Html {

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
            color: white;
            font-size: 1em;
            text-align: center;
          }

          .error {
            color: red;
            font-size: 1em;
            text-align: center;
          }
        "#
    ))
    .unwrap();

    let history = use_navigator().unwrap();

    let success_message = use_state(|| None::<String>);
    let error_message = use_state(|| None::<String>);
    let onsubmit = {

        let success_message = success_message.clone();
        let error_message = error_message.clone();

        Callback::from(move |user: User| {
            let history = history.clone();
            let success_message = success_message.clone();
            let error_message = error_message.clone();
            let user_data = SignUpData {
                username: user.username.to_string(),
                email: user.email.to_string(),
                password: user.password.to_string(),
            };
            // Inside the `onsubmit` callback
            spawn_local(async move {
                match api::create_account(user_data.username, user_data.password, user_data.email).await {
                    Ok(_auth_response) => {
                        success_message.set(Some("Sign-up successful! Redirecting...".to_string()));
                        history.push(&Route::ConfirmSignUp);  
                    },
                    Err(error) => {
                        // Make sure error is extracted correctly; adapt as needed for your error handling
                        let error_msg = error.as_string().unwrap_or_else(|| "Unknown error occurred".to_string());
                        error_message.set(Some(format!("Failed to sign up: {}", error_msg)));
                    }
                }
            });
        })
    };

    html! {
      <div class={stylesheet}>
        <h1>{"Create Account"}</h1>
        <section>
          <div>
            <AccountForm {onsubmit} action={Action::CreateAccount} />
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



