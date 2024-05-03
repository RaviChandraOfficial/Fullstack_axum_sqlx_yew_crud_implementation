use stylist::{yew::styled_component, Style};
use yew::prelude::*;

use wasm_bindgen_futures::spawn_local;
use reqwest::{Client, Error};
use serde::{Serialize, Deserialize};
use crate::components::molecules::otpverify::{AccountForm, Action, User};

#[derive(Serialize, Deserialize, Clone, PartialEq, Default)]
struct OtpConfirmation {
    pub username: String,
    pub otp: String,
}

#[styled_component(ConfirmSignUp)]
pub fn confirm_sign_up() -> Html {
    let stylesheet = Style::new(css!(
        r#"
        .form-group {
            margin-bottom: 1rem;
        }
        .input {
            padding: 0.5rem;
            border: 1px solid #ccc;
            border-radius: 0.25rem;
            width: 75%;
        }
        .button {
            background-color: #007b// A simple text input componentff;
            color: white;
            padding: 0.5rem 1rem;
            border: none;
            border-radius: 0.25rem;
            cursor: pointer;
        }
        .button:hover {
            background-color: #0056b3;
        }
        .error-message {
            color: red;
            font-size: 1em;
        }
        "#
    )).unwrap();

let success_message = use_state(|| None::<String>);
let error_message = use_state(|| None::<String>);


let onsubmit = {

    let success_message_clone = success_message.clone();
    let error_message_clone = error_message.clone();


    Callback::from(move |user: User| {

        let success_message_inner = success_message_clone.clone();
        let error_message_inner = error_message_clone.clone();

        let confirmation_data = OtpConfirmation {
            username: user.username,
            otp: user.otp,
        };


        spawn_local(async move {
            match confirm_otp(&confirmation_data).await {
                Ok(_) => {
                    success_message_inner.set(Some("OTP verified successfully!".to_string()));

                }
                Err(_) => {
                    error_message_inner.set(Some("OTP verification failed.".to_string()));
                }
            }
        });
    })
};

html! {
    <div class={stylesheet}>
      <h1>{"Sign Up Confirmation"}</h1>
      <section>
        <div>
          <AccountForm {onsubmit} action={Action::OtpVerify} />
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

async fn confirm_otp(otp_data: &OtpConfirmation) -> Result<(), Error> {
    let client = Client::new();
    let response = client.post("http://localhost:3000/signup_confirm")
                         .json(otp_data)
                         .send().await?
                         .error_for_status();
    response.map(|_| ())
}

