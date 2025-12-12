use crate::http::{HttpClient, HttpError};
use crate::model::user::{DataEditUser, DataUserProfile, FieldsUser, User, UserStatus};

pub struct EditUserBuilder<'a> {
    pub http: &'a HttpClient,
    pub target_id: String,    // We store the ID here!
    pub data: DataEditUser,   // We wrap your existing struct
}
impl<'a> EditUserBuilder<'a> {
    pub fn display_name(mut self, display_name: impl Into<String>) -> Self {
        self.data.display_name = Some(display_name.into());
        self
    }
    pub fn avatar(mut self, avatar: &str) -> Self {
        self.data.avatar = Some(avatar.to_string());
        self
    }
    pub fn status(mut self, status: UserStatus) -> Self {
        self.data.status = Some(status);
        self
    }
    pub fn profile(mut self, profile: DataUserProfile) -> Self {
        self.data.profile = Some(profile);
        self
    }
    pub fn remove(mut self, field: FieldsUser) -> Self {
        if let Some(ref mut vec) = self.data.remove {
            vec.push(field);
        } else {
            self.data.remove = Some(vec![field]);
        }
        self
    }
    pub async fn execute(self) -> Result<User, HttpError> {
        self.http.request_edit_user(&self.target_id, self.data).await
    }
}