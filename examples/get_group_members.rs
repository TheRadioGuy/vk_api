#[tokio::main]
async fn main() {
    let mut params = vkapi::Params::new();
    params
        .add("group_id", "194950468")
        .add("fields", "bdate,first_name,last_name");
    let mut vk_api = vkapi::VK::new("5.103", "en");
    vk_api.set_access_token(
        "db619ee5e3e018f25fc3a1d27153198c22d8c84850be604e3a2bb2d66a8a5d13771b56198d08ea3ac358e"
            .to_string(),
    );
    let response = vk_api
        .request("groups.getMembers", &mut params)
        .await
        .unwrap();
    response["response"]["items"].members().for_each(|user| {
        println!(
            "Name: {}, Surname: {}, Birth date: {}",
            user["first_name"], user["last_name"], user["bdate"]
        );
    });
}
