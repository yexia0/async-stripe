// ======================================
// This file was automatically generated.
// ======================================

use async_stripe_client::{
    ids::BitcoinTransactionId,
    params::{Object, Timestamp},
    resources::Currency,
};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "BitcoinTransaction".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BitcoinTransaction {
    /// Unique identifier for the object.
    pub id: BitcoinTransactionId,

    /// The amount of `currency` that the transaction was converted to in real-time.
    pub amount: i64,

    /// The amount of bitcoin contained in the transaction.
    pub bitcoin_amount: i64,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) to which this transaction was converted.
    pub currency: Currency,

    /// The receiver to which this transaction was sent.
    pub receiver: String,
}

impl Object for BitcoinTransaction {
    type Id = BitcoinTransactionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "bitcoin_transaction"
    }
}