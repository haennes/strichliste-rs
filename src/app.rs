use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde;
use serde::{Deserialize, Serialize};

use std::hash::DefaultHasher;
use std::hash::Hash;

#[derive(Debug, Clone, Deserialize, Serialize, Hash)]
pub struct User {
    name: String,
    nickname: String,
    money: i32,
}

#[server(GetUsers, "/api")]
pub async fn get_users(token: String) -> Result<Vec<User>, ServerFnError> {
    let mut users = Vec::<User>::new();
    for i in 0..20 {
        users.push(User {
            name: String::from(format!("Test-{}", i)),
            nickname: String::from(format!("Nick-{}", i)),
            money: i,
        });
    }
    Ok(users)
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/strichliste-rs.css"/>

        // sets the document title
        <Title text="Stichliste"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {

        <Await

            future=|| get_users(String::new())
            let:data
        >
            <div class="users">
                {data.to_owned().unwrap().into_iter()
                    .map(|user| view! {
                        <a class="user" href="#">
                            <p class="nickname">{user.nickname}</p>
                            <p class="user">{user.name}</p>
                            <p calss="money">{user.money} "€"</p>
                        </a>
                    }).collect_view()
                }
            </div>
        </Await>
    }
}
