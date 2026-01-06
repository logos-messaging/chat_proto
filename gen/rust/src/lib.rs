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
