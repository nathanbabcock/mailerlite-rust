use mailerlite_rs::{form::Form, response::Response, MailerLite};

#[tokio::main]
async fn main() {
    let api_key: String = String::from("Your MailerLite API Key");

    let mailerlite: MailerLite = MailerLite::new(api_key);

    let form: Form = Form::new().add("name", "Dummy Group");

    let response: Response = mailerlite.group().create(form.clone()).await;

    println!("{:#?}", response);
}
