//! Market module support
//!
//! <https://docs.terra.money/docs/develop/module-specifications/spec-market.html>

use crate::{proto, tx::Msg, AccountId, Coin, Error, ErrorReport, Result};

/// MsgDelegate represents a message to delegate coins to a validator.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgSwap {
    /// Address of the trader.
    pub trader: AccountId,

    /// Coin being offered.
    pub offer_coin: Coin,

    /// Denom of the coin to swap to.
    pub ask_denom: String,
}

impl Msg for MsgSwap {
    type Proto = proto::terra::market::v1beta1::MsgSwap;
}

impl TryFrom<proto::terra::market::v1beta1::MsgSwap> for MsgSwap {
    type Error = ErrorReport;

    fn try_from(proto: proto::terra::market::v1beta1::MsgSwap) -> Result<MsgSwap> {
        MsgSwap::try_from(&proto)
    }
}

impl TryFrom<&proto::terra::market::v1beta1::MsgSwap> for MsgSwap {
    type Error = ErrorReport;

    fn try_from(proto: &proto::terra::market::v1beta1::MsgSwap) -> Result<MsgSwap> {
        let offer_coin = proto
            .offer_coin
            .as_ref()
            .ok_or(Error::MissingField { name: "offer_coin" })?;

        Ok(MsgSwap {
            trader: proto.trader.parse()?,
            offer_coin: Coin {
                denom: offer_coin.denom.parse()?,
                amount: offer_coin.amount.parse()?,
            },
            ask_denom: proto.ask_denom.parse()?
        })
    }
}

impl From<MsgSwap> for proto::terra::market::v1beta1::MsgSwap {
    fn from(msg: MsgSwap) -> proto::terra::market::v1beta1::MsgSwap {
        proto::terra::market::v1beta1::MsgSwap::from(&msg)
    }
}

impl From<&MsgSwap> for proto::terra::market::v1beta1::MsgSwap {
    fn from(msg: &MsgSwap) -> proto::terra::market::v1beta1::MsgSwap {
        let offer_coin = proto::cosmos::base::v1beta1::Coin {
            denom: msg.offer_coin.denom.to_string(),
            amount: msg.offer_coin.amount.to_string(),
        };

        proto::terra::market::v1beta1::MsgSwap {
            trader: msg.trader.to_string(),
            offer_coin: Some(offer_coin),
            ask_denom: msg.ask_denom.to_string(),
        }
    }
}


/// MsgDelegate represents a message to delegate coins to a validator.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgSwapSend {
    /// Address of the offer coin payer.
    pub from_address: AccountId,

    /// Address of the recipient.
    pub to_address: AccountId,

    /// Coin being offered.
    pub offer_coin: Coin,

    /// Denom of the coin to swap to.
    pub ask_denom: String,
}

impl Msg for MsgSwapSend {
    type Proto = proto::terra::market::v1beta1::MsgSwapSend;
}

impl TryFrom<proto::terra::market::v1beta1::MsgSwapSend> for MsgSwapSend {
    type Error = ErrorReport;

    fn try_from(proto: proto::terra::market::v1beta1::MsgSwapSend) -> Result<MsgSwapSend> {
        MsgSwapSend::try_from(&proto)
    }
}

impl TryFrom<&proto::terra::market::v1beta1::MsgSwapSend> for MsgSwapSend {
    type Error = ErrorReport;

    fn try_from(proto: &proto::terra::market::v1beta1::MsgSwapSend) -> Result<MsgSwapSend> {
        let offer_coin = proto
            .offer_coin
            .as_ref()
            .ok_or(Error::MissingField { name: "offer_coin" })?;

        Ok(MsgSwapSend {
            from_address: proto.from_address.parse()?,
            to_address: proto.to_address.parse()?,
            offer_coin: Coin {
                denom: offer_coin.denom.parse()?,
                amount: offer_coin.amount.parse()?,
            },
            ask_denom: proto.ask_denom.parse()?
        })
    }
}

impl From<MsgSwapSend> for proto::terra::market::v1beta1::MsgSwapSend {
    fn from(msg: MsgSwapSend) -> proto::terra::market::v1beta1::MsgSwapSend {
        proto::terra::market::v1beta1::MsgSwapSend::from(&msg)
    }
}

impl From<&MsgSwapSend> for proto::terra::market::v1beta1::MsgSwapSend {
    fn from(msg: &MsgSwapSend) -> proto::terra::market::v1beta1::MsgSwapSend {
        let offer_coin = proto::cosmos::base::v1beta1::Coin {
            denom: msg.offer_coin.denom.to_string(),
            amount: msg.offer_coin.amount.to_string(),
        };

        proto::terra::market::v1beta1::MsgSwapSend {
            from_address: msg.from_address.to_string(),
            to_address: msg.to_address.to_string(),
            offer_coin: Some(offer_coin),
            ask_denom: msg.ask_denom.to_string(),
        }
    }
}