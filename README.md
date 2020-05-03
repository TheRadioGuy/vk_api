# VK API
![Crates.io](https://img.shields.io/crates/v/vkapi)

_It's in early stage, so we need your contribution_

### How to get access_token?
You can get it by 3 ways:
* Direct auth
  * call `direct_auth()` method
  * or by this link: `https://api.vk.com/oauth/token?grant_type=password&client_id=2274003&scope=notify,photos,friends,audio,video,notes,pages,docs,status,questions,offers,wall,groups,messages,notifications,stats,ads,offline&client_secret=hHbZxrka2uZ6jB1inYsH&username=YOUR_PHONE_OR_MAIL&password=YOUR_PASSWORD`
* Service token
  * Create you application [here](https://vk.com/apps?act=manage) and copy service token
* __Enter in your group, click Manage, in right menu click API usage and then create access_token__
Last way is a prefered way, because access_token of user has some limitations

### Quick Example
```rust
let mut params = vk_api::Params::new();
let params = params.add("group_id", "142102660").add("fields", "bdate"); // add params

let mut vk_api = vk_api::VK::new("5.103", "ru"); // 5.103 is api version
vk_api.set_access_token("ACCESS_TOKEN".into()); // Access token is your token (how to get it see above)
let response = vk_api.request("groups.getMembers", params).unwrap(); // call groups.getMembers method with our parametres
    for user in response["response"]["items"].members() {
        println!("Name: {}, Surname: {}, Birth Date: {}", user["first_name"], user["last_name"], user["bdate"]); // Print all users information
    }
```
### I want more examples!
[Here](https://github.com/DuckerMan/vk_api/tree/master/examples) you go!

### Plans

- [x] Release first version
- [x] Make direct auth
- [x] Add LongPolling api support
- [x] Make documentation
- [x] Add to crates.io
- [ ] Make code more readable
- [ ] Add Buttons support
- [ ] Ensure that this crate is added to the VK SDK
- [ ] Add graceful error handling
- [x] Make file loading more easier(**it works, but it still in progress**)
- [ ] Make macroses
### Thanks guys from Rust chat, namely:

@MikailBag, @ozkriff and [Bulat Idiatullin](https://vk.com/freeducker)