#![doc(html_logo_url = "https://avatars0.githubusercontent.com/u/7853871?s=128", html_favicon_url = "https://avatars0.githubusercontent.com/u/7853871?s=256", html_root_url = "http://ironframework.io/core/cookie")]
#![doc(html_logo_url = "https://avatars0.githubusercontent.com/u/7853871?s=128", html_favicon_url = "https://avatars0.githubusercontent.com/u/7853871?s=256", html_root_url = "http://ironframework.io/core/cookie")]
#![crate_name = "cookie"]
//! Cookie parsing/setting plugin for the [iron](https://github.com/iron/iron) framework.

extern crate iron;
extern crate plugin;
extern crate persistent;
extern crate modifier;
extern crate cookie;

pub use parser::*;
pub use cookie::*;

mod parser;
