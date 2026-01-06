// #![allow(clippy::all)]
// #![allow(warnings)]

pub mod wap {
    pub mod convos {
        pub mod private_v1 {
            include!("wap/convos/private_v1/wap.convos.private_v1.rs");
        }
    }

    pub mod encryption {
        include!("wap/encryption/wap.encryption.rs");
    }

    pub mod envelope {
        include!("wap/envelope/wap.envelope.rs");
    }

    pub mod inbox {
        include!("wap/inbox/wap.inbox.rs");
    }

    pub mod invite {
        include!("wap/invite/wap.invite.rs");
    }

    pub mod reliability {
        include!("wap/reliability/wap.reliability.rs");
    }
}

#[cfg(test)]
mod tests {
    use super::wap::{
        encryption::{encrypted_payload::Encryption, EncryptedPayload, Plaintext},
        inbox::{inbox_v1_frame::FrameType, InboxV1Frame, Note},
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
    fn test_inbox_frame_roundtrip() {
        let note = Note {
            text: "This is a test note".to_string(),
        };

        let frame = InboxV1Frame {
            recipient: "alice".to_string(),
            frame_type: Some(FrameType::Note(note.clone())),
        };

        let mut buf = Vec::new();
        frame.encode(&mut buf).expect("Encoding failed");

        let decoded = InboxV1Frame::decode(&buf[..]).expect("Decoding failed");

        match decoded.frame_type {
            Some(FrameType::Note(n)) => {
                assert_eq!(n.text, note.text);
            }
            _ => panic!("Expected Note variant"),
        }
    }

    #[test]
    fn test_invite_private_roundtrip() {
        let invite = InvitePrivateV1 {
            initiator: Bytes::from_static(b"initiator"),
            initiator_ephemeral: Bytes::from_static(b"ephemeral"),
            participant: Bytes::from_static(b"participant"),
            participant_ephemeral_id: 42,
            discriminator: "test_discriminator".to_string(),
            initial_message: None, // skipping encrypted payload for simplicity
        };

        let mut buf = Vec::new();
        invite.encode(&mut buf).expect("Encoding failed");

        let decoded = InvitePrivateV1::decode(&buf[..]).expect("Decoding failed");

        assert_eq!(decoded.initiator, Bytes::from_static(b"initiator"));
        assert_eq!(decoded.participant_ephemeral_id, 42);
        assert_eq!(decoded.discriminator, "test_discriminator");
    }
}
