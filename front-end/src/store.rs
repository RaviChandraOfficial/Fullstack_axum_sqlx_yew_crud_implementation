use crate::api::SigninResponse;
use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Clone, Serialize, Deserialize, Store, PartialEq, Debug)]
#[store(storage = "local")]
pub struct Store {
    pub token: String,
}


impl Default for Store {
    fn default() -> Self {
        Self {
            token: Default::default(),
        }
    }
}



pub struct PostCredentials{
    pub id: i32,
    pub name:String,
    pub location: String,
    pub data: String
}


// Updates the token in the store when a user logs in.
pub fn login_token(auth_response: &SigninResponse, dispatch: Dispatch<Store>) {

    dispatch.reduce_mut(move |store| {
        store.token = auth_response.access_token.clone();

    });
}


//Clears the user information and tasks from the store when a user logs out.
pub fn logout(dispatch: Dispatch<Store>) {
    dispatch.reduce_mut(|store| {
        store.token = String::new();
    });
}

