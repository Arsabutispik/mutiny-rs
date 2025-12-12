# Changelog

All notable changes to this project will be documented in this file.

## [0.5.0] - 2025-12-12

### 🔍 Other Changes

- Merge pull request #2

* feat: Properly define structs for user model

* fix: remove debugging logging

* fix: Rename Http to HttpClient

* feat: Add edit_user functionality

* feat: Add fetch_dms ([1e495e9](https://github.com/arsabutispik/mutiny-rs/commit/1e495e923f2a49c27309336fdd8b0f33e9b07dbc))

### 🚀 Features

- Use macro for routing to simplify routes ([3b30238](https://github.com/arsabutispik/mutiny-rs/commit/3b302383ec9afab882cc9b2bee536e3d057f5c39))

## [0.4.0] - 2025-11-26

### 🚜 Refactor

- **core:** Overhaul http routing and channel models ([81a3841](https://github.com/arsabutispik/mutiny-rs/commit/81a38410b630bc6691df075184e23df5c3524f4a))

## [0.3.2] - 2025-11-26

### 🐛 Bug Fixes

- Remove main.rs as it was for testing ([b72d068](https://github.com/arsabutispik/mutiny-rs/commit/b72d068694ce8a1e30d5e3da422f0fb417b99229))
- Make Group and ServerInvite public struct ([ddcfb06](https://github.com/arsabutispik/mutiny-rs/commit/ddcfb06e10483cc16b4a17fa6769993d296864d8))

## [0.3.0] - 2025-11-25

### 🔍 Other Changes

- Initial commit ([061f7f3](https://github.com/arsabutispik/mutiny-rs/commit/061f7f3e197b1bc658506e7299d4af3c5a4cf407))
- Change name ([92dc070](https://github.com/arsabutispik/mutiny-rs/commit/92dc07018104a3feeb26e24fffb4c8140b3c1a0a))
- Update github name ([5b5cb0b](https://github.com/arsabutispik/mutiny-rs/commit/5b5cb0b5aaca21ae29066fd95f90e1fe963f93ac))
- Update github name ([a607d85](https://github.com/arsabutispik/mutiny-rs/commit/a607d852731ac36cdefd3d5c2b8f3233115e50b9))
- Websocket and some events are added ([92625fb](https://github.com/arsabutispik/mutiny-rs/commit/92625fbfae3505d7f97440f14b7011eaab2a75ca))
- Websocket not pinging fixed, message reply and some more correct message fields are added ([e8fc9f1](https://github.com/arsabutispik/mutiny-rs/commit/e8fc9f178f371f5cfd14dabb37db9e208a1257a9))
- Update http.rs ([bf7294b](https://github.com/arsabutispik/mutiny-rs/commit/bf7294b3671ff1a0afb3bcd418b0889d95cc7440))
- Merge remote-tracking branch 'origin/master' ([68ebe15](https://github.com/arsabutispik/mutiny-rs/commit/68ebe15ba51b2442fc53d58f39eac20bd81b7421))
- Add more functionality to messages; send embeds and edit them, fix some http errors ([6323bd9](https://github.com/arsabutispik/mutiny-rs/commit/6323bd94784df2dba4d356c2c4d5daab1dd3c6c9))
- Channel create_message, fetch added reply and create_reply conjoined into shared functions ([7d1eee0](https://github.com/arsabutispik/mutiny-rs/commit/7d1eee0183a37674045fb3ca25b3f2e05e9c3cfb))
- Fixed errors and introduced bulk_delete on channel ([31c4b88](https://github.com/arsabutispik/mutiny-rs/commit/31c4b880b814574df6259e5aa3a29cc76a53c74e))
- Remove debug logs ([e19c8fd](https://github.com/arsabutispik/mutiny-rs/commit/e19c8fd3ae0d2cd6c316074bf7ba7421d68807fb))
- Fix relationship having an unnecessary amount of code ([2416a16](https://github.com/arsabutispik/mutiny-rs/commit/2416a160df58a78fca82ccb5f728ab1186b020ea))
- Refactor builders and modals to follow Stoat's API and Rust standards ([d30c32c](https://github.com/arsabutispik/mutiny-rs/commit/d30c32c7755249aef5895c7f9f21e0f61c14ecb2))
- Follow Rust standards by moving functions to context and http relatively ([2695080](https://github.com/arsabutispik/mutiny-rs/commit/2695080102c6973d60fa741609d1f142e777b504))
- Expand on channel ([c7d896d](https://github.com/arsabutispik/mutiny-rs/commit/c7d896daafec81e83c26e9f54dd44ac2a9a180d2))

### 🚜 Refactor

- **core:** [**breaking**] Unify HTTP layer and introduce Context; migrate builders and models ([56bcc42](https://github.com/arsabutispik/mutiny-rs/commit/56bcc4246151fc9ad2a81ebd7687378966b183bc))

