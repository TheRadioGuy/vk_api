# VK API

_It's in early stage, so we need your contribution_

### Yoo, how to get access_token?
You can get it by 3 ways:
* Direct auth
  * call `direct_auth()` method
  * or by this link: `https://api.vk.com/oauth/token?grant_type=password&client_id=2274003&scope=notify,photos,friends,audio,video,notes,pages,docs,status,questions,offers,wall,groups,messages,notifications,stats,ads,offline&client_secret=hHbZxrka2uZ6jB1inYsH&username=YOUR_PHONE_OR_MAIL&password=YOUR_PASSWORD`
* Service token
  * Create you application [here]( https://vk.com/apps?act=manage) and copy service token
* Enter in your group, click Manage, in right menu click API usage and then create access_token

### Quick Example
```rust
let mut params = HashMap::new(); // Our params
params.insert("group_id", "142102660");
params.insert("fields", "bdate");

let mut vk_api = vk_api::VK::new("5.103", "ru"); // 5.103 is api version
vk_api.set_access_token("ACCESS_TOKEN"); // Access token is your token (how to get it see above)
let response = vk_api.request("groups.getMembers", params).unwrap(); // call groups.getMembers method with our parametres
    for user in response["response"]["items"].members() {
        println!("Имя: {}, Фамилия: {}, Дата рождения: {}", user["first_name"], user["last_name"], user["bdate"]); // Print all users information
    }
```

### Plans

- [x] Release first version
- [x] Make direct auth
- [ ] Add LongPolling api support
- [ ] Make code more readable