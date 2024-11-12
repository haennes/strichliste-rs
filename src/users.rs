use leptos::{component, server, view, IntoView, Params, ReadSignal, ServerFnError, SignalGet};
use leptos_router::A;
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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct User {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) nickname: String,
    pub(crate) money: Euros,
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

#[cfg(feature = "debug_users")]
pub async fn get_users() -> Result<Vec<User>, ServerFnError> {
    Ok([
        User {
            id: 1,
            name: "a".into(),
            nickname: "na".into(),
            money: 300.into(),
        },
        User {
            id: 1,
            name: "b".into(),
            nickname: "nb".into(),
            money: 450.into(),
        },
        User {
            id: 1,
            name: "c".into(),
            nickname: "nc".into(),
            money: (-650).into(),
        },
    ]
    .into())
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
