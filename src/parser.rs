//! Parsing functionality - get cookie data

use std::io::Error;
use modifier::Modifier;
use iron::typemap::Key;
use iron::{Request, Response};
use iron::headers::Cookie;
use plugin::*;
use persistent::{Read, Write};
use cookie::CookieJar;

pub struct Cookies(pub CookieJar<'static>);
pub struct CookieKey(pub &'static [u8]);

impl Key for Cookies { type Value = Option<Cookie>; }
impl Key for CookieKey { type Value = &'static [u8]; }

impl<'a, 'b> Plugin<Request<'a, 'b>> for Cookies {
    type Error = Error;

    fn eval(req: &mut Request) -> Result<Option<Cookie>, Error> {
        Ok(match req.headers.get::<Cookie>().clone() {
            Some(cookies) => Some(cookies.clone()),
            _ => None
        })
    }
}

impl Cookies {
    pub fn get_cookie_jar(req: &mut Request) -> CookieJar<'static> {
        let key = req.get::<Read<CookieKey>>().ok().unwrap();
        let mut jar = CookieJar::new(&key);

        let cookies: &Option<Cookie> = req.get_ref::<Cookies>().ok().unwrap();
        match cookies {
            &Some::<Cookie>(ref cookies) => {
                for cookie in cookies.iter() {
                    jar.add_original(cookie.clone());
                }
            },
            _ => ()
        };

        jar
    }
}

impl<'a, 'b> Modifier<Request<'a, 'b>> for CookieKey {
    fn modify(self, req: &mut Request) {
        let mutex = req.get::<Write<CookieKey>>().ok().unwrap();
        let mut cookie_key = mutex.lock().unwrap();

        *cookie_key = self.0;
    }
}

impl Modifier<Response> for Cookies {
    fn modify(self, res: &mut Response) {
        let cookie = Cookie::from_cookie_jar(&self.0);
        res.headers.set(cookie);
    }
}
