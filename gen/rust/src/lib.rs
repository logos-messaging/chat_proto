// #![allow(clippy::all)]
// #![allow(warnings)]

pub mod logoschat {
    pub mod convos {
        pub mod private_v1 {
            include!("logoschat/convos/private_v1/logoschat.convos.private_v1.rs");
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
        encryption::{encrypted_payload::Encryption, EncryptedPayload, Plaintext},
        invite::InvitePrivateV1,
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
}
