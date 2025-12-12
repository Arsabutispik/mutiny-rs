pub mod user;
pub mod ready;
pub mod channel;
pub mod embed;
pub mod file;
pub mod message;
pub mod invite;
pub mod traits;
pub mod permissions;


/// Utility function to check if a boolean value is false
pub fn if_false(t: &bool) -> bool {
    !t
}

/// Utility function to check if an u32 is zero
pub fn if_zero_u32(t: &u32) -> bool {
    t == &0
}

/// Utility function to check if an option doesnt contain true
pub fn if_option_false(t: &Option<bool>) -> bool {
    t != &Some(true)
}