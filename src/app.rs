use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde;

use super::users::*;

#[cfg(feature = "ssr")]
pub mod ssr {
    use leptos::server_fn::ServerFnError;
    use sqlx::postgres::{PgPool, PgPoolOptions};
    //use sqlx::ConnectOptions;
    //use sqlx::Pool;

    pub async fn db() -> Result<PgPool, ServerFnError> {
        Ok(PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://strichliste-rs:secret@localhost/strichliste-rs")
            .await?)
        //let opts = PgConnectOptions::new()
        //    .socket("/var/lib/postgresql")
        //    .username("strichliste-rs")
        //    .password("secret")
        //    .database("strichliste-rs");
        //Ok(PgPool::connect_with(opts).await?)
    }
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
                    <Route path="/" view=HomePage/>
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
                {
                    data
                        .to_owned()
                        .unwrap_or_default()
                        .into_iter()
                    .map(|user| view! {
                        <UserTile user=create_signal(user).0 />
                    }).collect_view()
                }
            </div>
        </Await>
    }
}
