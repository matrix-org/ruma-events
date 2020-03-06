//! "To-device" versions of encrypted and key verification events.
//!
//! Each "to-device" event includes only the `content`, `type`, and `sender`
//! fields. To-device events are sent directly from one device to the other
//! without the need to create a room.

use ruma_identifiers::UserId;
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};
use serde_json::Value;

use crate::{
    forwarded_room_key::ForwardedRoomKeyEventContent, room::encrypted::EncryptedEventContent,
    room_key_request::RoomKeyRequestEventContent,
    key::verification::{
        start::StartEventContent,
        accept::AcceptEventContent,
        cancel::CancelEventContent,
        request::RequestEventContent,
        key::KeyEventContent,
        mac::MacEventContent,
    },
    room_key::RoomKeyEventContent, util::get_field, TryFromRaw,
};

/// To-device versions of events that will appear in the to-device part of a
/// sync response.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[allow(clippy::large_enum_variant)]
pub enum ToDevice {
    /// To-device version of the *m.room_key* event.
    RoomKey(ToDeviceRoomKey),
    /// To-device version of the *m.room.encrypted* event.
    RoomEncrypted(ToDeviceEncrypted),
    /// To-device version of the *m.forwarded_room_key* event.
    ForwardedRoomKey(ToDeviceForwardedRoomKey),
    /// To-device version of the *m.room_key_request* event.
    RoomKeyRequest(ToDeviceRoomKeyRequest),
    /// To-device version of the *m.key.verification.start* event.
    KeyVerificationStart(ToDeviceVerificationStart),
    /// To-device version of the *m.key.verification.accept* event.
    KeyVerificationAccept(ToDeviceVerificationAccept),
    /// To-device version of the *m.key.verification.key* event.
    KeyVerificationKey(ToDeviceVerificationKey),
    /// To-device version of the *m.key.verification.mac* event.
    KeyVerificationMac(ToDeviceVerificationMac),
    /// To-device version of the *m.key.verification.cancel* event.
    KeyVerificationCancel(ToDeviceVerificationCancel),
    /// To-device version of the *m.key.verification.request* event.
    KeyVerificationRequest(ToDeviceVerificationRequest),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
/// To-device event.
pub struct ToDeviceEvent<C> {
    /// The unique identifier for the user who sent this event.
    pub sender: UserId,
    /// Data specific to the event type.
    pub content: C,
}

/// To-device version of the *m.room_key* event.
pub type ToDeviceRoomKey = ToDeviceEvent<RoomKeyEventContent>;

/// To-device version of the *m.room.encrypted* event.
pub type ToDeviceEncrypted = ToDeviceEvent<EncryptedEventContent>;

/// To-device version of the *m.forwarded_room_key* event.
pub type ToDeviceForwardedRoomKey = ToDeviceEvent<ForwardedRoomKeyEventContent>;

/// To-device version of the *m.room_key_request* event.
pub type ToDeviceRoomKeyRequest = ToDeviceEvent<RoomKeyRequestEventContent>;

/// To-device version of the *m.key.verification.start* event.
pub type ToDeviceVerificationStart = ToDeviceEvent<StartEventContent>;

/// To-device version of the *m.key.verification.accept* event.
pub type ToDeviceVerificationAccept = ToDeviceEvent<AcceptEventContent>;

/// To-device version of the *m.key.verification.key* event.
pub type ToDeviceVerificationKey = ToDeviceEvent<KeyEventContent>;

/// To-device version of the *m.key.verification.mac* event.
pub type ToDeviceVerificationMac = ToDeviceEvent<MacEventContent>;

/// To-device version of the *m.key.verification.cancel* event.
pub type ToDeviceVerificationCancel = ToDeviceEvent<CancelEventContent>;

/// To-device version of the *m.key.verification.request* event.
pub type ToDeviceVerificationRequest = ToDeviceEvent<RequestEventContent>;

impl TryFromRaw for ToDevice {
    type Raw = raw::ToDevice;
    type Err = String;

    fn try_from_raw(raw: raw::ToDevice) -> Result<Self, Self::Err> {
        use crate::util::try_convert_variant as conv;
        use raw::ToDevice::*;

        match raw {
            RoomKey(c) => conv(ToDevice::RoomKey, c),
            RoomEncrypted(c) => conv(ToDevice::RoomEncrypted, c),
            ForwardedRoomKey(c) => conv(ToDevice::ForwardedRoomKey, c),
            RoomKeyRequest(c) => conv(ToDevice::RoomKeyRequest, c),
            KeyVerificationStart(c) => conv(ToDevice::KeyVerificationStart, c),
            KeyVerificationAccept(c) => conv(ToDevice::KeyVerificationAccept, c),
            KeyVerificationKey(c) => conv(ToDevice::KeyVerificationKey, c),
            KeyVerificationMac(c) => conv(ToDevice::KeyVerificationMac, c),
            KeyVerificationCancel(c) => conv(ToDevice::KeyVerificationCancel, c),
            KeyVerificationRequest(c) => conv(ToDevice::KeyVerificationRequest, c),
        }
    }
}

impl<C> TryFromRaw for ToDeviceEvent<C>
where
    C: TryFromRaw,
{
    type Raw = ToDeviceEvent<C::Raw>;
    type Err = C::Err;

    fn try_from_raw(raw: ToDeviceEvent<C::Raw>) -> Result<Self, Self::Err> {
        Ok(Self {
            content: C::try_from_raw(raw.content)?,
            sender: raw.sender,
        })
    }
}

impl<'de, C> Deserialize<'de> for ToDeviceEvent<C>
where
    C: DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // TODO: Optimize, what should be optimized here? Can we expand this
        // comment?
        let value = Value::deserialize(deserializer)?;

        Ok(Self {
            content: get_field(&value, "content")?,
            sender: get_field(&value, "sender")?,
        })
    }
}

mod raw {
    use serde::{Deserialize, Deserializer};
    use serde_json::Value;

    use super::ToDeviceEvent;
    use crate::{
        key::verification::{
            start::raw::StartEventContent,
            accept::raw::AcceptEventContent,
            cancel::raw::CancelEventContent,
            request::raw::RequestEventContent,
            key::raw::KeyEventContent,
            mac::raw::MacEventContent,
        },
        forwarded_room_key::raw::ForwardedRoomKeyEventContent,
        room_key_request::raw::RoomKeyRequestEventContent,
        room::encrypted::raw::EncryptedEventContent, room_key::raw::RoomKeyEventContent,
        util::get_field,
    };

    /// To-device version of the *m.room_key* event.
    pub type ToDeviceRoomKey = ToDeviceEvent<RoomKeyEventContent>;
    /// To-device version of the *m.room.encrypted* event.
    pub type ToDeviceEncrypted = ToDeviceEvent<EncryptedEventContent>;
    /// To-device version of the *m.forwarded_room_key* event.
    pub type ToDeviceForwardedRoomKey = ToDeviceEvent<ForwardedRoomKeyEventContent>;
    /// To-device version of the *m.room_key_request* event.
    pub type ToDeviceRoomKeyRequest = ToDeviceEvent<RoomKeyRequestEventContent>;
    /// To-device version of the *m.key.verification.start* event.
    pub type ToDeviceVerificationStart = ToDeviceEvent<StartEventContent>;
    /// To-device version of the *m.key.verification.accept* event.
    pub type ToDeviceVerificationAccept = ToDeviceEvent<AcceptEventContent>;
    /// To-device version of the *m.key.verification.key* event.
    pub type ToDeviceVerificationKey = ToDeviceEvent<KeyEventContent>;
    /// To-device version of the *m.key.verification.mac* event.
    pub type ToDeviceVerificationMac = ToDeviceEvent<MacEventContent>;
    /// To-device version of the *m.key.verification.cancel* event.
    pub type ToDeviceVerificationCancel = ToDeviceEvent<CancelEventContent>;
    /// To-device version of the *m.key.verification.request* event.
    pub type ToDeviceVerificationRequest = ToDeviceEvent<RequestEventContent>;

    /// A stripped-down version of a state event that is included along with some other events.
    #[derive(Clone, Debug)]
    #[allow(clippy::large_enum_variant)]
    pub enum ToDevice {
        /// To-device version of the *m.room_key* event.
        RoomKey(ToDeviceRoomKey),
        /// To-device version of the *m.room.encrypted* event.
        RoomEncrypted(ToDeviceEncrypted),
        /// To-device version of the *m.forwarded_room_key* event.
        ForwardedRoomKey(ToDeviceForwardedRoomKey),
        /// To-device version of the *m.room_key_request* event.
        RoomKeyRequest(ToDeviceRoomKeyRequest),
        /// To-device version of the *m.key.verification.start* event.
        KeyVerificationStart(ToDeviceVerificationStart),
        /// To-device version of the *m.key.verification.accept* event.
        KeyVerificationAccept(ToDeviceVerificationAccept),
        /// To-device version of the *m.key.verification.key* event.
        KeyVerificationKey(ToDeviceVerificationKey),
        /// To-device version of the *m.key.verification.mac* event.
        KeyVerificationMac(ToDeviceVerificationMac),
        /// To-device version of the *m.key.verification.cancel* event.
        KeyVerificationCancel(ToDeviceVerificationCancel),
        /// To-device version of the *m.key.verification.request* event.
        KeyVerificationRequest(ToDeviceVerificationRequest),
    }

    impl<'de> Deserialize<'de> for ToDevice {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            use crate::{util::try_variant_from_value as from_value, EventType::*};
            use serde::de::Error as _;

            // TODO: Optimize, what should be optimized here? Can we expand this
            // comment?
            let value = Value::deserialize(deserializer)?;
            let event_type = get_field(&value, "type")?;

            match event_type {
                RoomKey => from_value(value, ToDevice::RoomKey),
                RoomEncrypted => from_value(value, ToDevice::RoomEncrypted),
                ForwardedRoomKey => from_value(value, ToDevice::ForwardedRoomKey),
                RoomKeyRequest => from_value(value, ToDevice::RoomKeyRequest),
                KeyVerificationStart => from_value(value, ToDevice::KeyVerificationStart),
                KeyVerificationAccept => from_value(value, ToDevice::KeyVerificationAccept),
                KeyVerificationKey => from_value(value, ToDevice::KeyVerificationKey),
                KeyVerificationMac => from_value(value, ToDevice::KeyVerificationMac),
                KeyVerificationCancel => from_value(value, ToDevice::KeyVerificationCancel),
                KeyVerificationRequest => from_value(value, ToDevice::KeyVerificationRequest),
                _ => Err(D::Error::custom("unknown to-device event")),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // TODO add tests for all this.
}
