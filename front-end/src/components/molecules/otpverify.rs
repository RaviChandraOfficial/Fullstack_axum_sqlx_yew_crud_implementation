use std::ops::Deref;

use crate::components::atoms::{
    bb_button::BBButton,
    bb_text_input::{BBTextInput, InputType},
};
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<User>,
    pub action: Action,
}

#[derive(Clone, PartialEq)]
pub enum Action {
    OtpVerify
}

impl ToString for Action {
    fn to_string(&self) -> String {
        match self {
            Action::OtpVerify=>"Otpverify",
        }
        .to_owned()
    }
}
#[derive(Default, Clone)]
pub struct User {
    pub username: String,
    pub otp: String,

}

#[styled_component(AccountForm)]
pub fn account_form(props: &Props) -> Html {
    let state = use_state(User::default);

    let username_onchange = {
        let state = state.clone();
        Callback::from(move |username: String| {
            let mut user = state.deref().clone();
            user.username = username;
            state.set(user);
        })
    };

    let otp = {
        let state = state.clone();
        Callback::from(move |otp: String| {
            let mut user = state.deref().clone();
            user.otp = otp;
            state.set(user);
        })
    };

    let onsubmit = {
        let onsubmit_prop = props.onsubmit.clone();
        let state = state;
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let user = state.deref().clone();
            onsubmit_prop.emit(user);
        })
    };

    html! {
      <form {onsubmit}>
        <BBTextInput data_test="username" label="Username" placeholder="What username do you want?" class="input" input_type={InputType::Text} onchange={username_onchange} />
        <BBTextInput data_test="otp" label="OTP" placeholder="Enter Your otp" class="input" input_type={InputType::Password} onchange={otp} />
        <BBButton label={props.action.to_string()} data_test="submit" />
      </form>
    }
}




