// use leptos::{
//     component, create_memo, create_signal, server, view, with, IntoView, Params, ReadSignal,
//     ServerFnError, SignalGet, SignalWith,
// };
use leptos::*;
use leptos_router::{use_params, Params, A};
use serde::{Deserialize, Serialize};

use crate::{Euros, MoneyColor};

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow, sqlx::Decode))]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct DBUser {
    id: i32,
    name: String,
    nickname: String,
    money: i32,
}

impl From<DBUser> for User {
    fn from(value: DBUser) -> Self {
        User {
            id: value.id,
            name: value.name,
            nickname: value.nickname,
            money: value.money.into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct User {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) nickname: String,
    pub(crate) money: Euros,
}

impl User {
    fn error() -> User {
        User {
            id: 0,
            name: "FAILURE".to_string(),
            nickname: "FAILURE".to_string(),
            money: 0.into(),
        }
    }
}

#[derive(Params, PartialEq)]
struct UserParam {
    id: Option<i32>,
}
#[component]
pub(crate) fn UserTile(#[prop(into)] user: ReadSignal<User>) -> impl IntoView {
    let money = user.get().money;
    view! {
    <A class="user transplink" href=move || format!("{}", user.get().id)>
        <p class="nickname">{user.get().nickname}</p>
        <p class="user">{user.get().name}</p>
        <MoneyColor money=money/>
    </A>
    }
}

pub(crate) fn UserPage() -> impl IntoView {
    let param = use_params::<UserParam>();

    let user_id = move || {
        param.with(|params| {
            params
                .as_ref()
                .map(|params| params.id.unwrap())
                .unwrap_or_default()
        })
    };
    let user = move || get_user(user_id());
    let get_user_data =
        |user_data: &Result<User, ServerFnError>| user_data.clone().unwrap_or(User::error());
    view! {
        <Await
          future = move || user()
          let:user_data
        >

        <div>
          <p class="username_userpage">{get_user_data(user_data).nickname}</p>
          <div class="bar">
            // <p class="bar_member"> {get_user_data(user_data).money.to_string()}</p>
            // <p class="bar_member"> {get_user_data(user_data).money.to_string()}</p>
            <div class="bar_member" style="width:20%">
              <MoneyColor money=get_user_data(user_data).money/>
            </div>
            <div class="bar_member" style="min-width:20%">
              <A href= "payout" class = "transplink">
              Auszahlen
              </A>
            </div>
            <div class="bar_member" style="min-width:20%">
              <A href= "transfer" class = "transplink">
              Überweisen
              </A>
            </div>
            <div class="bar_member" style="min-width:20%">
              <A href= "deposit" class = "transplink">
              Überweisen
              </A>
            </div>
          </div>
        </div>
        </Await>
                // <p>"Hello User with id " {user_id} </p>
           //<p class="username_userpage">{move || get_user(user_id).name}</p>
           // <p class="username_userpage">{ get_user(user_id().unwrap()).await.unwrap().name}</p>

           // <p class="username_userpage">{  move || {get_user(user_id.unwrap()).unwrap().name}}</p>
          // <p class="username_userpage">{move || user.nickname}</p>
          // <p class="username_userpage">{move  || block_on(user()).unwrap().name} </p>
    }
}

#[cfg(feature = "debug_users")]
pub async fn get_users() -> Result<Vec<User>, ServerFnError> {
    let mut v = Vec::new();
    for i in 0..10 {
        v.push(User {
            id: i,
            name: format!("User {i}"),
            nickname: format!("Nickname {i}"),
            money: (50 * i * (-1 as i32).pow(i as u32)).into(),
        })
    }
    Ok(v)
    // Ok([
    //     User {
    //         id: 1,
    //         name: "a".into(),
    //         nickname: "na".into(),
    //         money: 300.into(),
    //     },
    //     User {
    //         id: 1,
    //         name: "b".into(),
    //         nickname: "nb".into(),
    //         money: 450.into(),
    //     },
    //     User {
    //         id: 1,
    //         name: "c".into(),
    //         nickname: "nc".into(),
    //         money: (-650).into(),
    //     },
    // ]
    // .into())
}

#[cfg_attr(not(feature = "debug_users"), server(GetUsers))]
#[cfg(not(feature = "debug_users"))]
pub async fn get_users() -> Result<Vec<User>, ServerFnError> {
    use crate::app::ssr::*;
    use futures_util::TryStreamExt;

    let mut users = Vec::<User>::new();

    let conn = db().await?;

    let mut rows = sqlx::query_as::<_, DBUser>("select * from users").fetch(&conn);
    while let Some(row) = rows.try_next().await? {
        users.push(row.into());
    }

    Ok(users)
}

#[server(GetUser)]
pub async fn get_user(user_id: i32) -> Result<User, ServerFnError> {
    let user = User {
        id: user_id,
        name: String::from(format!("Test-{}", user_id)),
        nickname: String::from(format!("Nick-{}", user_id)),
        money: user_id.into(),
    };
    Ok(user)
}
