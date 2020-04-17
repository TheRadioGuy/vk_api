fn main() {
    println!("Hello, world!");
    let mut params = vk_api::Params::new();
    let params = params.add("group_id", "142102660").add("fields", "bdate");
    let mut vk_api = vk_api::VK::new("5.103", "ru");
    vk_api.set_access_token(
        "ADD_YOUR_TOKEN"
            .to_string(),
    );
    // let response = vk_api.request("groups.getMembers", params).unwrap();
    // for user in response["response"]["items"].members() {
    //     println!(
    //         "Имя: {}, Фамилия: {}, Дата рождения: {}",
    //         user["first_name"], user["last_name"], user["bdate"]
    //     );
    // }

    vk_api.start_longpoll(142102660, 25, Box::new(|event| {
        println!("callback, event: {} ", event);
    }));
}
пш