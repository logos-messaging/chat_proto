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
}

#[cfg(test)]
mod tests {
    use super::logoschat::{
        encryption::{encrypted_payload::Encryption, EncryptedPayload, Plaintext},
        invite::InvitePrivateV1,
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
}
