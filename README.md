# VK API
[![](http://meritbadge.herokuapp.com/vkapi)](https://crates.io/crates/vkapi)
[![documentation (docs.rs)](https://docs.rs/vkapi/badge.svg)](https://docs.rs/vkapi)

_It's in early stage, so we need your contribution_

### README is outdated, now we are rewriting it


### How to get access_token?
You can get it by 3 ways:
* Direct auth
  * call `direct_auth()` method
  * or by this link: `https://api.vk.com/oauth/token?grant_type=password&client_id=2274003&scope=notify,photos,friends,audio,video,notes,pages,docs,status,questions,offers,wall,groups,messages,notifications,stats,ads,offline&client_secret=hHbZxrka2uZ6jB1inYsH&username=YOUR_PHONE_OR_MAIL&password=YOUR_PASSWORD`
* Service token
  * Create you application [here](https://vk.com/apps?act=manage) and copy service token
* __Enter in your group, click Manage, in right menu click API usage and then create access_token__
Last way is a prefered way, because access_token of user has some limitations

### Quick Guide
Your best friends are `param!` macro and `request` method!

```rust
let mut vk_api = vkapi::VK::new("5.103", "en");
vk_api.set_access_token("token".into());
let mut params = param!{"group_id" => "194950468", "fields" => "bdate,first_name,last_name"};
let response = vk_api
        .request("groups.getMembers", &mut params)
        .await
        .unwrap();
```


### I want examples!
[Here](https://github.com/DuckerMan/vk_api/tree/master/examples) you go!

### Plans

- [x] Release first version
- [x] Make direct auth
- [x] Add LongPolling api support
- [x] Make documentation
- [x] Add to crates.io
- [ ] Make code more readable
- [ ] Add Buttons support
- [ ] Ensure that this crate is added to the VK SDK site
- [ ] Add graceful error handling
- [x] Make file loading more easier(**it works, but it still in progress**)
- [x] Make macroses
### Thanks guys from Rust chat, namely:

@MikailBag, @ozkriff and [Bulat Idiatullin](https://vk.com/freeducker)