use crate::builders::edit_user::EditUserBuilder;
use crate::http::routing::Route;
use crate::http::{HttpClient, HttpError};
use crate::model::channel::Channel;
use crate::model::user::{DataEditUser, User};

impl HttpClient {
    pub async fn fetch_self(&self) -> Result<User, HttpError> {
        let route = Route::FetchMe;
        self.get::<User>(route).await
    }
    pub async fn fetch_user(&self, id: &str) -> Result<User, HttpError> {
        let route = Route::FetchUser { user_id: &id };
        self.get::<User>(route).await
    }
    pub fn edit_user<'a>(&'a self, user_id: impl Into<String>) -> EditUserBuilder<'a> {
        EditUserBuilder {
            http: self,
            target_id: user_id.into(),
            data: DataEditUser::new(),
        }
    }
    pub async fn request_edit_user(&self, user_id: &str, data: DataEditUser) -> Result<User, HttpError> {
        let route = Route::EditUser { user_id };
        self.execute::<DataEditUser, User>(route, data).await
    }
    pub async fn fetch_dms(&self) -> Result<Vec<Channel>, HttpError> {
        let route = Route::FetchMe;
        self.get(route).await
    }
}