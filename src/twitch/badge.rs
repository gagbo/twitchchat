/// BadgeKind are the `kind` of badges that are associated with messages.
///
/// Any unknown (e.g. custom badges/sub events, etc) are placed into the
/// `Unknown` variant
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BadgeKind {
    /// Admin badge
    Admin,
    /// Bits badge
    Bits,
    /// Broadcaster badge
    Broadcaster,
    /// Global moderator badge
    GlobalMod,
    /// Channel moderator badge
    Moderator,
    /// Subscriber badge
    Subscriber,
    /// Twitch staff badge
    Staff,
    /// Turbo badge
    Turbo,
    /// Twitch Prime badge
    Premium,
    /// VIP Badge
    VIP,
    /// Partner badge
    Partner,
    /// An Unknown badge
    Unknown(String),
    // Reserve the right to add more fields to this enum
    #[doc(hidden)]
    __Nonexhaustive,
}

/// Badges attached to a message
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Badge {
    /// The kind of Badge
    pub kind: BadgeKind,
    /// Any associated data with the badge
    ///
    /// May be the version, the number of bits, the number of months needed for the subscriber badge, etc.
    pub data: String,
}

impl Badge {
    pub(crate) fn parse(input: &str) -> Option<Self> {
        use BadgeKind::*;
        let input = input.to_ascii_lowercase();
        let mut iter = input.split('/');
        let kind = match iter.next()? {
            "admin" => Admin,
            "bits" => Bits,
            "broadcaster" => Broadcaster,
            "global_mod" => GlobalMod,
            "moderator" => Moderator,
            "subscriber" => Subscriber,
            "staff" => Staff,
            "turbo" => Turbo,
            "premium" => Premium,
            "vip" => VIP,
            "partner" => Partner,
            badge => Unknown(badge.to_string()),
        };
        Some(Badge {
            kind,
            data: iter.next()?.to_string(),
        })
    }
}

/// Metadata related to the chat badges.
// TODO verify that this is mirrored
pub type BadgeInfo = Badge;
