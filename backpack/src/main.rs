mod surf2;

use crate::surf2::Response;

fn main() {
    let res = surf2::get("https://httpbin.org/get").unwrap();

    dbg!(res.body());
    dbg!(res.headers());
    dbg!(res.status());
}
