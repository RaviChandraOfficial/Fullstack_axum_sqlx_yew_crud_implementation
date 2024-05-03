use crate::pages::get_data::Getdata;
use crate::pages::post_data::PostData;
use crate::pages::logout::Logout;
use crate::pages::put_data::PutData;
use crate::pages::delete::DeleteData;
use crate::pages::{create_account::CreateAccount, home::Home, login::Login, confirmsignup::ConfirmSignUp};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/create-account")]
    CreateAccount,
    #[at("/confirmsignup")]
    ConfirmSignUp,
    #[at("/signin")]
    Login, 
    #[at("/signout")]
    Logout, 
    #[at("/get/user")]
    Getdata, 
    #[at("/post")]
    PostData,
    #[at("/put")]
    PutData,
    #[at("/delete")]
    DeleteData,

}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::CreateAccount => html! { <CreateAccount /> },
        Route::ConfirmSignUp => html! { <ConfirmSignUp/> },
        Route::Login => html! { <Login /> },
        Route::Getdata => html! { <Getdata /> },
        Route::Logout => html! { <Logout /> },
        Route::PostData => html! { <PostData/> },
        Route::PutData => html! { <PutData /> },
        Route::DeleteData => html! { <DeleteData /> },
    }
}
