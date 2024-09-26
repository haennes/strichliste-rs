use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde;
use serde::{Deserialize, Serialize};
use server_fn::codec::FromReq;

use std::hash::DefaultHasher;
use std::hash::Hash;

#[cfg(feature = "ssr")]
pub mod ssr {
    use leptos::server_fn::ServerFnError;
    use sqlx::postgres::{PgPool, PgPoolOptions};
    use sqlx::Pool;

    pub async fn db() -> Result<PgPool, ServerFnError> {
        Ok(PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://strichliste-rs@localhost/strichliste-rs")
            .await?)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    id: i32,
    name: String,
    nickname: String,
    money: i32,
}

#[derive(Params, PartialEq)]
struct UserParam {
    id: Option<i32>,
}

#[server(GetUsers)]
pub async fn get_users() -> Result<Vec<User>, ServerFnError> {
    use self::ssr::*;
    use futures_util::stream::TryStreamExt;

    let mut users = Vec::<User>::new();

    let mut conn = db().await?;

    let mut rows = sqlx::query_as::<_, User>("select * from users").fetch(&mut conn);
    while let Some(row) = rows.try_next().await? {
        users.push(row);
    }

    Ok(users)
}

#[server(GetUser)]
pub async fn get_user(user_id: i32) -> Result<User, ServerFnError> {
    let user = User {
        id: user_id,
        name: String::from(format!("Test-{}", user_id)),
        nickname: String::from(format!("Nick-{}", user_id)),
        money: user_id,
    };

    Ok(user)
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
                    <Route path="/:id" view=UserPage/>
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

            future=|| get_users()
            let:data
        >
            <div class="users">
                {data.to_owned().unwrap().into_iter()
                    .map(|user| view! {
                        <A class="user" href=move || format!("{}", user.id)>
                            <p class="nickname">{user.nickname}</p>
                            <p class="user">{user.name}</p>
                            <p calss="money">{user.money} "€"</p>
                        </A>
                    }).collect_view()
                }
            </div>
        </Await>
    }
}

#[component]
fn UserPage() -> impl IntoView {
    let param = use_params::<UserParam>();
    let user_id =
        move || param.with(|params| params.as_ref().map(|params| params.id).unwrap_or_default());
    view! {
        <p>"Hello User with id " {user_id} </p>
    }
}
