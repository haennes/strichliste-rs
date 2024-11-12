use std::fmt::Display;

use leptos::*;
use serde::{Deserialize, Serialize};
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow, sqlx::Decode))]
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Euros(i32);

impl From<i32> for Euros {
    fn from(i: i32) -> Self {
        Euros(i)
    }
}

impl Euros {
    fn new(euros: u16, cents: u8) -> Self {
        Euros((euros * 100 + cents as u16).into())
    }
}

impl Display for Euros {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}€", self.0 / 100, (self.0 % 100).abs())
        //Ok(())
    }
}
#[component]
pub(crate) fn MoneyColor(money: Euros) -> impl IntoView {
    view! {
        <div class="money">
          <p class = move || {if money < 0.into() { "red" }else {"green" }} >{money.to_string()}</p>
        </div>
    }
}
