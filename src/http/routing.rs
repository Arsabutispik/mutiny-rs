use reqwest::Method;

macro_rules! api_routes {
    (
        $(
            $variant:ident
            $( { $($arg:ident : $typ:ty),* $(,)? } )?
            => $method:ident, $path:literal
            $(, $fmt_arg:ident)*
        );* $(;)?
    ) => {
        pub enum Route<'a> {
            $(
                $variant $( { $($arg: $typ),* } )?,
            )*
        }

        impl<'a> Route<'a> {
            #[allow(unused_variables)]
            pub fn method(&self) -> Method {
                match self {
                    $(
                        Self::$variant $( { $($arg),* } )? => Method::$method,
                    )*
                }
            }

            pub fn path(&self) -> String {
                match self {
                    $(
                        Self::$variant $( { $($arg),* } )? => format!($path, $($fmt_arg),*),
                    )*
                }
            }
        }
    };
}
api_routes! {
    // --- Channel Operations ---
    GetChannel      { channel_id: &'a str } => GET,    "/channels/{}", channel_id;
    EditChannel     { channel_id: &'a str } => PATCH,  "/channels/{}", channel_id;
    DeleteChannel   { channel_id: &'a str } => DELETE, "/channels/{}", channel_id;

    // --- Invites & Recipients ---
    CreateInvite    { channel_id: &'a str }
                    => POST,   "/channels/{}/invites", channel_id;

    AddRecipient    { channel_id: &'a str, user_id: &'a str }
                    => PUT,    "/channels/{}/recipients/{}", channel_id, user_id;

    RemoveRecipient { channel_id: &'a str, user_id: &'a str }
                    => DELETE, "/channels/{}/recipients/{}", channel_id, user_id;

    // --- Messages ---
    SendMessage     { channel_id: &'a str }
                    => POST,   "/channels/{}/messages", channel_id;

    FetchMessages   { channel_id: &'a str }
                    => GET,    "/channels/{}/messages", channel_id;

    EditMessage     { channel_id: &'a str, message_id: &'a str }
                    => PATCH,  "/channels/{}/messages/{}", channel_id, message_id;

    MessageDelete   { channel_id: &'a str, message_id: &'a str }
                    => DELETE, "/channels/{}/messages/{}", channel_id, message_id;

    MessagePin      { channel_id: &'a str, message_id: &'a str }
                    => POST,   "/channels/{}/messages/{}/pin", channel_id, message_id;

    MessageUnpin    { channel_id: &'a str, message_id: &'a str }
                    => DELETE, "/channels/{}/messages/{}/pin", channel_id, message_id;

    // --- User Operations ---
    FetchMe         => GET,    "/users/@me";
    FetchDMs        => GET,    "/dms/";

    FetchUser       { user_id: &'a str }    => GET,    "/users/{}", user_id;
    EditUser        { user_id: &'a str }    => PATCH,  "/users/{}", user_id;
    BlockUser       { user_id: &'a str }    => PUT,    "/users/{}/block", user_id;
    FetchProfile    { user_id: &'a str }    => GET,    "/users/{}/profile", user_id;
    FetchUserFlags  { user_id: &'a str }    => GET,    "/users/{}/flags", user_id;
    FindMutual      { user_id: &'a str }    => GET,    "/users/{}/mutual", user_id;
    GetDefaultAvatar { user_id: &'a str }   => GET,    "/users/{}/default", user_id;
    OpenDM          { user_id: &'a str }    => GET,   "/users/{}/dm", user_id;
    RemoveFriend    { user_id: &'a str }    => DELETE, "/users/{}/friend", user_id;
    UnblockUser     { user_id: &'a str }    => DELETE, "/users/{}/block", user_id;
}