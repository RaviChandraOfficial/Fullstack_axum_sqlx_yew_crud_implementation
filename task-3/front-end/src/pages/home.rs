use crate::store::Store;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yewdux::prelude::*;
use yew_router::prelude::Link;use std::ops::Deref;
use crate::router::Route;
#[styled_component(Home)]
pub fn component() -> Html {
    let stylesheet = Style::new(css!(
        r#"
        .section-container {
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            width: 90%;
            height: 70vh;
        }

        .link-section {
            display: flex;
            justify-content: center;
            align-items: center;
            padding: 20px;
            margin: 10px;
            width: 50%;
            border-radius: 30px;
        }
        
        a {
            font-size: 24px; 
            padding: 15px 30px; 
            background-color: yellow;
            color: black; 
            text-decoration: none;
            border-radius: 5px;
            width: 100%;
            text-align: center;
        }
        
        a:hover {
            background-color: #0056b3;
            color: white;
        }
        "#
    ))
    .unwrap();

    let (store, _dispatch) = use_store::<Store>();

 
    let token = store.token.clone();

    html! {
        <>
        if is_logged_in(&token){
        <div class={stylesheet}>
            <div class="section-container">
                <section class="link-section" style="background-color: #f0f8ff;"> // Light blue background
                    <Link<Route> to={Route::Getdata}>{ "GET" }</Link<Route>>
                </section>
                <section class="link-section" style="background-color: #f5fffa;"> // Mint cream background
                    <Link<Route> to={Route::PostData}>{ "POST" }</Link<Route>>
                </section>
                <section class="link-section" style="background-color: #fffacd;"> // Lemon chiffon background
                    <Link<Route> to={Route::PutData}>{ "PUT" }</Link<Route>>
                </section>
                <section class="link-section" style="background-color: #ffe4e1;"> // Misty rose background
                    <Link<Route> to={Route::DeleteData}>{ "DELETE" }</Link<Route>>
                </section>
            </div>
        </div>
        }
        </>
    }
}





fn is_logged_in(token: &str) -> bool {
    !token.is_empty()
}

