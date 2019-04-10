use std::ops::Deref;

#[derive(Clone, Debug)]
pub(crate) enum MessageStrategy {
    NamedProto(String),
    Proto,
    Json,
    Plaintext,
}

impl MessageStrategy {
    pub(crate) fn content_type(&self) -> &'static str {
        use MessageStrategy::*;

        match *self {
            NamedProto(_) | Proto => "application/protobuf",
            Json => "application/json",
            Plaintext => "text/plain",
        }
    }
}

// Extensions rely on TypeIds and simple aliases appear to have the same TypeId
// as the things they alias, so we'll (ab)use the newtype pattern:
#[derive(Clone, Debug)]
pub(crate) struct MessageParseStrategy(pub MessageStrategy);

impl Deref for MessageParseStrategy {
    type Target = MessageStrategy;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for MessageParseStrategy {
    fn default() -> Self {
        Self(MessageStrategy::Plaintext)
    }
}

impl From<MessageStrategy> for MessageParseStrategy {
    fn from(strat: MessageStrategy) -> Self {
        Self(strat)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct MessageEncodeStrategy(pub MessageStrategy);

impl Deref for MessageEncodeStrategy {
    type Target = MessageStrategy;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<MessageStrategy> for MessageEncodeStrategy {
    fn from(strat: MessageStrategy) -> Self {
        Self(strat)
    }
}

impl Default for MessageEncodeStrategy {
    fn default() -> Self {
        Self(MessageStrategy::Plaintext)
    }
}
