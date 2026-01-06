# Chat Protobuf Definitionss

This repository contains the canonical **protobuf definitions** for the WAP (Waku Application Protocol) used by LibChat and related components.

It is **schema-only**:
- No application logic
- Stable, versioned wire formats
- Intended to be consumed by multiple languages

Generated bindings (e.g. Rust) are produced from these schemas.

---

## Prerequisites

This project uses [Buf](https://buf.build) for protobuf linting and code generation.

### Install Buf

On macOS and Linux:
```sh
brew install bufbuild/buf/buf
```

For other platforms, See https://buf.build/docs/cli/installation/.

## Repository Structure

```
protos/         # Protobuf source files
buf.yaml        # Buf module configuration
buf.gen.yaml    # Code generation configuration
gen/
  └── rust/     # Generated Rust bindings (prost)
```

## Generate Rust bindings

```
buf generate
```

This will generate Rust code under: `gen/rust/`, The generated crate can be used directly as a dependency in Rust projects.

## Usage (Rust)

Add in Cargo.toml:
```
chat-proto = { git = "https://github.com/logos-messaging/chat_proto" }
```

Example import:
```
use chat_proto::wap::{
    inbox::InboxV1Frame,
    invite::InvitePrivateV1,
    encryption::EncryptedPayload,
};
```
