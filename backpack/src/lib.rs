pub mod surf2;

/// Multiplies two integers
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod test {
    use super::*;
    use demonstrate::demonstrate;

    demonstrate! {
        describe "module" {
            use super::*;

            before {
                let four = 4;
            }

            it "can fail" {
                assert_eq!(multiply(2, 2), four);
            }

            test "is returnable" -> Result<(), &'static str> {
                if multiply(2, 2) == four {
                    Ok(())
                } else {
                    Err("It isn't 4! :o")
                }
            }

            #[async_attributes::test]
            async context "asynchronous" {
                before {
                    let is_4_task = async_std::task::spawn(async {
                        multiply(2, 2)
                    });
                }

                it "awaits" {
                    assert_eq!(four, is_4_task.await)
                }
            }
        }

        describe "surf2" {
            use super::*;
            use crate::surf2::{get, Response};

            test "HTTP get request" {
                let res = surf2::get("https://httpbin.org/get").unwrap();

                assert!(res.body().len() > 0);
                assert!(res.headers().keys().len() > 0);
                assert_eq!(res.headers()["content-type"], "application/json");
                assert_eq!(res.status(), 0);
            }
        }
    }
}
