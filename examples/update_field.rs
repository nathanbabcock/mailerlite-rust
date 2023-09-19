use mailerlite_rs::{form::Form, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let id: String = String::from("Your Field ID");

    let form: Form = Form::new().add("name", "Dummy Field");

    let response: Response = mailerlite.field().update(id, form.clone()).await;

    println!("{:#?}", response);
}
