# VK API

_It's in early stage, so we need your contribution_

### Yoo, how to get access_token?
You can get it by 3 ways:
* Direct auth
  * call `direct_auth()` method
  * or by this link: `https://api.vk.com/oauth/token?grant_type=password&client_id=2274003&scope=notify,photos,friends,audio,video,notes,pages,docs,status,questions,offers,wall,groups,messages,notifications,stats,ads,offline&client_secret=hHbZxrka2uZ6jB1inYsH&username=YOUR_PHONE_OR_MAIL&password=YOUR_PASSWORD`
* Service token
  * Create you application [Here]( https://vk.com/apps?act=manage) and copy service token
* Enter in your group, click Manage, in right menu click API usage and then create access_token
