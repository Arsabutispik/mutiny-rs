use crate::builders::edit_user::EditUserBuilder;
use crate::http::routing::Route;
use crate::http::{HttpClient, HttpError};
use crate::model::channel::Channel;
use crate::model::user::{DataEditUser, FlagResponse, MutualResponse, User, UserProfile};

impl HttpClient {
    /// # Fetch Self
    ///
    /// Retrieve your user information.
    pub async fn fetch_self(&self) -> Result<User, HttpError> {
        let route = Route::FetchMe;
        self.get::<User>(route).await
    }
    /// # Fetch User
    ///
    /// Retrieve a user's information.
    pub async fn fetch_user(&self, id: impl Into<String>) -> Result<User, HttpError> {
        let route = Route::FetchUser { user_id: &id.into() };
        self.get::<User>(route).await
    }
    /// # Edit User
    ///
    /// Edit currently authenticated user.
    pub fn edit_user<'a>(&'a self, user_id: impl Into<String>) -> EditUserBuilder<'a> {
        EditUserBuilder {
            http: self,
            target_id: user_id.into(),
            data: DataEditUser::new(),
        }
    }
    pub async fn request_edit_user(&self, user_id: impl Into<String>, data: DataEditUser) -> Result<User, HttpError> {
        let route = Route::EditUser { user_id: &user_id.into() };
        self.execute::<DataEditUser, User>(route, data).await
    }
    /// # Fetch Direct Message Channels
    ///
    /// This fetches your direct messages, including any DM and group DM conversations.
    pub async fn fetch_dms(&self) -> Result<Vec<Channel>, HttpError> {
        let route = Route::FetchMe;
        self.get(route).await
    }
    /// # Block User
    ///
    /// Block another user by their id.
    pub async fn block_user(&self, user_id: impl Into<String>) -> Result<User, HttpError> {
        let route = Route::BlockUser { user_id: &user_id.into() };
        self.request::<(), (), User>(route, None, None).await
    }
    /// # Fetch User Profile
    ///
    /// Retrieve a user's profile data.
    ///
    /// Will fail if you do not have permission to access the other user's profile.
    pub async fn fetch_profile(&self, user_id: impl Into<String>) -> Result<UserProfile, HttpError> {
        let route = Route::FetchProfile { user_id: &user_id.into() };
        self.get(route).await
    }
    /// # Fetch User Flags
    ///
    /// Retrieve a user's flags.
    pub async fn fetch_user_flags(&self, user_id: impl Into<String>) -> Result<FlagResponse, HttpError> {
        let route = Route::FetchUserFlags { user_id: &user_id.into() };
        self.get(route).await
    }
    /// # Fetch Mutual Friends, Servers, Groups and DMs
    ///
    /// Retrieve a list of mutual friends, servers, groups and DMs with another user.
    pub async fn find_mutual(&self, user_id: impl Into<String>) -> Result<MutualResponse, HttpError> {
        let route = Route::FindMutual { user_id: &user_id.into() };
        self.get(route).await
    }
    /// # Open Direct Message
    ///
    /// Open a DM with another user.
    ///
    /// If the target is oneself, a saved messages channel is returned.
    pub async fn open_dm(&self, user_id: impl Into<String>) -> Result<Channel, HttpError> {
        let route = Route::OpenDM { user_id: &user_id.into() };
        self.get(route).await
    }
    /// # Deny Friend Request / Remove Friend
    ///
    /// Denies another user's friend request or removes an existing friend.
    pub async fn remove_friend(&self, user_id: impl Into<String>) -> Result<User, HttpError> {
        let route = Route::RemoveFriend { user_id: &user_id.into() };
        self.get(route).await
    }
    /// # Unblock User
    ///
    /// Unblock another user by their id.
    pub async fn unblock_user(&self, user_id: impl Into<String>) -> Result<User, HttpError> {
        let route = Route::UnblockUser { user_id: &user_id.into() };
        self.get(route).await
    }
}