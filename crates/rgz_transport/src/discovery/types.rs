use rgz_msgs as msgs;

pub(crate) type DiscoveryPublisher = msgs::discovery::Publisher;

pub(crate) type DiscoveryMsgPublisher = msgs::discovery::publisher::MessagePublisher;
pub(crate) type DiscoverySrvPublisher = msgs::discovery::publisher::ServicePublisher;
pub(crate) type DiscoveryFlags = msgs::discovery::Flags;
pub(crate) type DiscoveryScope = msgs::discovery::publisher::Scope;
pub(crate) type DiscoveryPubType = msgs::discovery::publisher::PubType;
pub(crate) type DiscoveryDiscContents = msgs::discovery::DiscContents;
pub(crate) type DiscoveryMsg = msgs::Discovery;
pub(crate) type DiscoveryType = msgs::discovery::Type;
pub(crate) type DiscoverySubscriber = msgs::discovery::Subscriber;
