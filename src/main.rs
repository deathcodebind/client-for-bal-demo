use client::query::{signIn, sign_in};
use graphql_client::GraphQLQuery;
slint::include_modules!();

fn main() -> Result<(), anyhow::Error>{
    let ui = SignInWindow::new()?;
    ui.on_send_request({
        let ui_ = ui.as_weak();
        move |account, password| {
            let query = signIn::build_query(sign_in::Variables{
                account: account.to_string(),
                password: password.to_string(),
            });
            let client = reqwest::blocking::Client::new();
            let res = client.post("http://127.0.0.1:4000/graphql").json(&query).send().unwrap().json::<sign_in::ResponseData>();
            match res {
                Ok(res) => {
                    ui_.unwrap().set_response(res.sign_in.to_owned().unwrap().into());
                },
                Err(err) => {
                    ui_.unwrap().set_response(err.to_string().into());
                },
            }
        }
    });
    ui.run()?;
    Ok(())
}

