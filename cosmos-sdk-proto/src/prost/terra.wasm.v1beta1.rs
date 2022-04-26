/// MsgExecuteContract submits the given message data to a smart contract
/// https://docs.terra.money/docs/develop/module-specifications/spec-wasm.html#msgexecutecontract
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExecuteContract {
    /// Sender is the that actor that signed the messages
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// Contract is the address of the smart contract
    #[prost(string, tag = "2")]
    pub contract: ::prost::alloc::string::String,
    /// Msg json encoded message to be passed to the contract
    #[prost(bytes = "vec", tag = "3")]
    pub execute_msg: ::prost::alloc::vec::Vec<u8>,
    /// Funds coins that are transferred to the contract on execution
    #[prost(message, repeated, tag = "5")]
    pub coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
