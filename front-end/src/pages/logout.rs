use stylist::yew::styled_component;
use stylist::Style;
use yew::prelude::*;
use yew::html;
use crate::store;
use crate::store::Store;
use yewdux::functional::use_store;



#[styled_component(Logout)]
pub fn logout() -> Html {
    let stylesheet = Style::new(css!(
        r#"
          section {
            display: flex;
            justify-content: center;
          }

          section > div {
            width: 75vw;
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

    let (_store, store_dispatch) = use_store::<Store>();
    let success_message = use_state(|| None::<String>);
    let error_message = use_state(|| None::<String>);

    let on_logout = {
        let store_dispatch = store_dispatch.clone();
        let success_message = success_message.clone();
        let (_store, dispatch) = use_store::<Store>();
        
        Callback::from(move |_| {
            // Clear any stored authentication data
            let dispatch = dispatch.clone();
            store_dispatch.set(Store::default());
            store::logout(dispatch);
            success_message.set(Some("Logout successful!".to_string()));
        })
    };

    html! {
      <div class={stylesheet}>
        <h1>{"Logout"}</h1>
        <section>
          <div>
            <button onclick={on_logout}>{"Logout"}</button>
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
