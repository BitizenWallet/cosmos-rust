/// MsgSwap represents a message to swap coin to another denom.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSwap {
    #[prost(string, tag="1")]
    pub trader: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub offer_coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag="3")]
    pub ask_denom: ::prost::alloc::string::String,
}

/// MsgSwapSend represents a message to swap coin and send all result coin to recipient
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSwapSend {
    #[prost(string, tag="1")]
    pub from_address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub to_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub offer_coin: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag="4")]
    pub ask_denom: ::prost::alloc::string::String,
}
