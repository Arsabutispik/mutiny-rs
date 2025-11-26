use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct Permissions: u64 {
        // * Generic permissions
        /// Manage the channel or channels on the server
        const MANAGE_CHANNEL = 1 << 0;
        /// Manage the server
        const MANAGE_SERVER = 1 << 1;
        /// Manage permissions on servers or channels
        const MANAGE_PERMISSIONS = 1 << 2;
        /// Manage roles on server
        const MANAGE_ROLE = 1 << 3;
        /// Manage server customisation (includes emoji)
        const MANAGE_CUSTOMISATION = 1 << 4;

        // % 1 bit reserved

        // * Member permissions
        /// Kick other members below their ranking
        const KICK_MEMBERS = 1 << 6;
        /// Ban other members below their ranking
        const BAN_MEMBERS = 1 << 7;
        /// Timeout other members below their ranking
        const TIMEOUT_MEMBERS = 1 << 8;
        /// Assign roles to members below their ranking
        const ASSIGN_ROLES = 1 << 9;
        /// Change own nickname
        const CHANGE_NICKNAME = 1 << 10;
        /// Change or remove other's nicknames below their ranking
        const MANAGE_NICKNAMES = 1 << 11;
        /// Change own avatar
        const CHANGE_AVATAR = 1 << 12;
        /// Remove other's avatars below their ranking
        const REMOVE_AVATARS = 1 << 13;

        // % 7 bits reserved

        // * Channel permissions
        /// View a channel
        const VIEW_CHANNEL = 1 << 20;
        /// Read a channel's past message history
        const READ_MESSAGE_HISTORY = 1 << 21;
        /// Send a message in a channel
        const SEND_MESSAGE = 1 << 22;
        /// Delete messages in a channel
        const MANAGE_MESSAGES = 1 << 23;
        /// Manage webhook entries on a channel
        const MANAGE_WEBHOOKS = 1 << 24;
        /// Create invites to this channel
        const INVITE_OTHERS = 1 << 25;
        /// Send embedded content in this channel
        const SEND_EMBEDS = 1 << 26;
        /// Send attachments and media in this channel
        const UPLOAD_FILES = 1 << 27;
        /// Masquerade messages using custom nickname and avatar
        const MASQUERADE = 1 << 28;
        /// React to messages with emojis
        const REACT = 1 << 29;

        // * Voice permissions
        /// Connect to a voice channel
        const CONNECT = 1 << 30;
        /// Speak in a voice call
        const SPEAK = 1 << 31;
        /// Share video in a voice call
        const VIDEO = 1 << 32;
        /// Mute other members with lower ranking in a voice call
        const MUTE_MEMBERS = 1 << 33;
        /// Deafen other members with lower ranking in a voice call
        const DEAFEN_MEMBERS = 1 << 34;
        /// Move members between voice channels
        const MOVE_MEMBERS = 1 << 35;
        /// Listen to other users
        const LISTEN = 1 << 36;

        // * Channel permissions two electric boogaloo
        /// Mention everyone and online members
        const MENTION_EVERYONE = 1 << 37;
        /// Mention roles
        const MENTION_ROLES = 1 << 38;
    }
}