#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

extern crate chrono;

use rocket_contrib::serve::{StaticFiles};
use rocket_contrib::json::Json;
use chrono::NaiveDateTime;

#[get("/")]
fn index() -> &'static str {
  "Hello, world!"
}

#[get("/detail")]
fn detail() -> &'static str {
  "Foo, world!"
}

mod other {
  #[get("/world")]
  pub fn world() -> &'static str {
    "Hello, world!"
  }

  #[get("/hello/<name>")]
  pub fn hello(name: String) -> String {
    format!("Hello, {}!", name)
  }
}

#[derive(Serialize, Deserialize)]
struct Contract {
  starts_on: i64,
  ends_on: i64
}

#[derive(Serialize, Deserialize)]
struct Payload {
  amount: u64,
  extra: u64,
  contract: Contract,
  complete: bool
}

#[derive(Serialize, Deserialize)]
struct Payslip {
  amount: u64,
  complete: bool,
  duration: i64
}

struct Processor {
    payload: Payload // FIXME: json here?
}

impl Processor {
    fn process (&self) -> Payslip {
        let starts_on = NaiveDateTime::from_timestamp(self.payload.contract.starts_on, 0);
        let ends_on = NaiveDateTime::from_timestamp(self.payload.contract.ends_on, 0);

        let difference = ends_on.signed_duration_since(starts_on);

        Payslip {
            amount: self.payload.amount + self.payload.extra + 1000,
            duration: difference.num_days(),
            complete: self.payload.complete
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process() {
        let payload = Payload {
            amount: 3300,
            extra: 1000,
            contract: Contract {
                starts_on: 1553869190,
                ends_on: 1557890190
            },
            complete: true
        };

        let processor = Processor { payload: payload };
        let payslip = processor.process();

        assert_eq!(payslip.amount, 5300);
    }
}

#[post("/payslip", format = "json", data = "<payload>")]
fn payslip(payload: Json<Payload>) -> Json<Payslip> {
    let processor = Processor {
        payload: payload.into_inner()
    };

  Json(processor.process())
}

fn main() {
  rocket::ignite()
    .mount("/", routes![
      index,
      detail,
      payslip,
      other::world,
      other::hello
    ])
    .mount("/public", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
    .launch();
}
