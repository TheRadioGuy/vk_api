fn main() {
    let mut params = vk_api::Params::new();
    let params = params.add("group_id", "142102660").add("fields", "bdate");
    let mut vk_api = vk_api::VK::new("5.103", "ru");
    vk_api.set_access_token("YOUR_TOKEN".to_string());
    let response = vk_api.request("groups.getMembers", params).unwrap();
    response["response"]["items"].members().for_each(|user| {
        println!(
            "Name: {}, Surname: {}, Birth date: {}",
            user["first_name"], user["last_name"], user["bdate"]
        );
    });
}
