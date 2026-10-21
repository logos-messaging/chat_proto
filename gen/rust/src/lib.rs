// #![allow(clippy::all)]
// #![allow(warnings)]

pub mod logoschat {
    pub mod convos {
        pub mod private_v1 {
            include!("logoschat/convos/private_v1/logoschat.convos.private_v1.rs");
        }

        pub mod group_v2 {
            include!("logoschat/convos/group_v2/logoschat.convos.group_v2.rs");
        }
    }

    pub mod encryption {
        include!("logoschat/encryption/logoschat.encryption.rs");
    }

    pub mod envelope {
        include!("logoschat/envelope/logoschat.envelope.rs");
    }

    pub mod inbox {
        include!("logoschat/inbox/logoschat.inbox.rs");
    }

    pub mod invite {
        include!("logoschat/invite/logoschat.invite.rs");
    }

    pub mod reliability {
        include!("logoschat/reliability/logoschat.reliability.rs");
    }

    pub mod intro {
        include!("logoschat/intro/logoschat.intro.rs");
    }

    pub mod store {
        include!("logoschat/store/logoschat.store.rs");
    }
}

#[cfg(test)]
mod tests {
    use super::logoschat::{
        convos::group_v2::{group_v2_frame, GroupV2Frame},
        encryption::{encrypted_payload::Encryption, EncryptedPayload, Plaintext},
        inbox::{inbox_v2_frame, InboxV2Frame},
        invite::{GroupV1HeavyInvite, InvitePrivateV1},
        store::{AccountSubmissionV1, KeyPackageSubmissionV1},
    };
    use bytes::Bytes;
    use prost::Message;

    #[test]
    fn test_encrypted_payload_roundtrip() {
        let payload = EncryptedPayload {
            encryption: Some(Encryption::Plaintext(Plaintext {
                payload: Bytes::from_static(b"hello world"),
            })),
        };

        // Encode to bytes
        let mut buf = Vec::new();
        payload.encode(&mut buf).expect("Encoding failed");

        // Decode back
        let decoded = EncryptedPayload::decode(&buf[..]).expect("Decoding failed");

        match decoded.encryption {
            Some(Encryption::Plaintext(p)) => {
                assert_eq!(p.payload, Bytes::from_static(b"hello world"));
            }
            _ => panic!("Expected plaintext variant"),
        }
    }

    #[test]
    fn test_invite_private_roundtrip() {
        let invite = InvitePrivateV1 {
            discriminator: "test_discriminator".to_string(),
            initial_message: None, // skipping encrypted payload for simplicity
        };

        let mut buf = Vec::new();
        invite.encode(&mut buf).expect("Encoding failed");

        let decoded = InvitePrivateV1::decode(&buf[..]).expect("Decoding failed");

        assert_eq!(decoded.discriminator, "test_discriminator");
    }

    /// The store verifies `signature` over the received `payload` bytes, so
    /// both must survive a roundtrip byte-for-byte.
    #[test]
    fn test_keypackage_submission_roundtrip() {
        let submission = KeyPackageSubmissionV1 {
            device_id: Bytes::from_static(&[7u8; 32]),
            payload: Bytes::from_static(b"timestamp-and-keypackage"),
            signature: Bytes::from_static(&[9u8; 64]),
        };

        let mut buf = Vec::new();
        submission.encode(&mut buf).expect("Encoding failed");

        let decoded = KeyPackageSubmissionV1::decode(&buf[..]).expect("Decoding failed");

        assert_eq!(decoded, submission);
    }

    #[test]
    fn test_account_submission_roundtrip() {
        let submission = AccountSubmissionV1 {
            account_pub: Bytes::from_static(&[3u8; 32]),
            payload: Bytes::from_static(b"lamport-and-device-keys"),
            signature: Bytes::from_static(&[5u8; 64]),
        };

        let mut buf = Vec::new();
        submission.encode(&mut buf).expect("Encoding failed");

        let decoded = AccountSubmissionV1::decode(&buf[..]).expect("Decoding failed");

        assert_eq!(decoded, submission);
    }

    /// Each payload is a `google.protobuf.BytesValue`: a message of its own,
    /// whose field 1 holds the bytes.
    #[test]
    fn test_group_v2_frame_payload_is_a_bytes_value() {
        let frame = GroupV2Frame {
            sender_app_id: Bytes::from_static(b"app"),
            payload: Some(group_v2_frame::Payload::DeMlsWrapper(b"inner".to_vec())),
        };

        let mut buf = Vec::new();
        frame.encode(&mut buf).expect("Encoding failed");

        assert_eq!(buf, b"\x12\x07\x0a\x05inner\x22\x03app");

        let decoded = GroupV2Frame::decode(&buf[..]).expect("Decoding failed");

        assert_eq!(decoded, frame);
    }

    #[test]
    fn test_inbox_v2_frame_roundtrip() {
        let frame = InboxV2Frame {
            payload: Some(inbox_v2_frame::Payload::GroupV1(GroupV1HeavyInvite {
                welcome_bytes: Bytes::from_static(b"welcome"),
            })),
        };

        let mut buf = Vec::new();
        frame.encode(&mut buf).expect("Encoding failed");

        let decoded = InboxV2Frame::decode(&buf[..]).expect("Decoding failed");

        assert_eq!(decoded, frame);
    }
}
