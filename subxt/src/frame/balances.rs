// Copyright 2019-2020 Parity Technologies (UK) Ltd.
// This file is part of substrate-subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with substrate-subxt.  If not, see <http://www.gnu.org/licenses/>.

//! Implements support for the pallet_balances module.

use crate::frame::system::{
    System,
    SystemEventsDecoder,
};
use codec::{
    Decode,
    Encode,
};
use core::marker::PhantomData;
use frame_support::Parameter;
use sp_runtime::traits::{
    AtLeast32Bit,
    MaybeSerialize,
    Member,
};
use std::fmt::Debug;

/// The subset of the `pallet_balances::Trait` that a client must implement.
#[module]
pub trait Balances: System {
    /// The balance of an account.
    type Balance: Parameter
        + Member
        + AtLeast32Bit
        + codec::Codec
        + Default
        + Copy
        + MaybeSerialize
        + Debug
        + From<<Self as System>::BlockNumber>;
}

/// All balance information for an account.
#[derive(Clone, Debug, Eq, PartialEq, Default, Decode, Encode)]
pub struct AccountData<Balance> {
    /// Non-reserved part of the balance. There may still be restrictions on this, but it is the
    /// total pool what may in principle be transferred, reserved and used for tipping.
    ///
    /// This is the only balance that matters in terms of most operations on tokens. It
    /// alone is used to determine the balance when in the contract execution environment.
    pub free: Balance,
    /// Balance which is reserved and may not be used at all.
    ///
    /// This can still get slashed, but gets slashed last of all.
    ///
    /// This balance is a 'reserve' balance that other subsystems use in order to set aside tokens
    /// that are still 'owned' by the account holder, but which are suspendable.
    pub reserved: Balance,
    /// The amount that `free` may not drop below when withdrawing for *anything except transaction
    /// fee payment*.
    pub misc_frozen: Balance,
    /// The amount that `free` may not drop below when withdrawing specifically for transaction
    /// fee payment.
    pub fee_frozen: Balance,
}

/// The total issuance of the balances module.
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct TotalIssuanceStore<T: Balances> {
    #[store(returns = T::Balance)]
    /// Runtime marker.
    pub _runtime: PhantomData<T>,
}

/// Transfer some liquid free balance to another account.
///
/// `transfer` will set the `FreeBalance` of the sender and receiver.
/// It will decrease the total issuance of the system by the `TransferFee`.
/// If the sender's account is below the existential deposit as a result
/// of the transfer, the account will be reaped.
#[derive(Clone, Debug, PartialEq, Call, Encode)]
pub struct TransferCall<'a, T: Balances> {
    /// Destination of the transfer.
    pub to: &'a <T as System>::Address,
    /// Amount to transfer.
    #[codec(compact)]
    pub amount: T::Balance,
}

/// Transfer event.
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct TransferEvent<T: Balances> {
    /// Account balance was transfered from.
    pub from: <T as System>::AccountId,
    /// Account balance was transfered to.
    pub to: <T as System>::AccountId,
    /// Amount of balance that was transfered.
    pub amount: T::Balance,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        error::{
            Error,
            RuntimeError,
        },
        events::EventsDecoder,
        extrinsic::{
            PairSigner,
            Signer,
        },
        subscription::EventSubscription,
        system::AccountStoreExt,
        tests::{
            test_client,
            TestRuntime,
        },
    };
    use sp_core::{
        sr25519::Pair,
        Pair as _,
    };
    use sp_keyring::AccountKeyring;

    #[async_std::test]
    async fn test_basic_transfer() {
        env_logger::try_init().ok();
        let alice = PairSigner::<TestRuntime, _>::new(AccountKeyring::Alice.pair());
        let bob = PairSigner::<TestRuntime, _>::new(AccountKeyring::Bob.pair());
        let (client, _) = test_client().await;

        let alice_pre = client.account(alice.account_id(), None).await.unwrap();
        let bob_pre = client.account(bob.account_id(), None).await.unwrap();

        let event = client
            .transfer_and_watch(&alice, &bob.account_id(), 10_000)
            .await
            .unwrap()
            .transfer()
            .unwrap()
            .unwrap();
        let expected_event = TransferEvent {
            from: alice.account_id().clone(),
            to: bob.account_id().clone(),
            amount: 10_000,
        };
        assert_eq!(event, expected_event);

        let alice_post = client.account(alice.account_id(), None).await.unwrap();
        let bob_post = client.account(bob.account_id(), None).await.unwrap();

        assert!(alice_pre.data.free - 10_000 >= alice_post.data.free);
        assert_eq!(bob_pre.data.free + 10_000, bob_post.data.free);
    }

    #[async_std::test]
    async fn test_state_total_issuance() {
        env_logger::try_init().ok();
        let (client, _) = test_client().await;
        let total_issuance = client.total_issuance(None).await.unwrap();
        assert_ne!(total_issuance, 0);
    }

    #[async_std::test]
    async fn test_state_read_free_balance() {
        env_logger::try_init().ok();
        let (client, _) = test_client().await;
        let account = AccountKeyring::Alice.to_account_id();
        let info = client.account(&account, None).await.unwrap();
        assert_ne!(info.data.free, 0);
    }

    #[async_std::test]
    async fn test_transfer_error() {
        env_logger::try_init().ok();
        let alice = PairSigner::new(AccountKeyring::Alice.pair());
        let hans = PairSigner::new(Pair::generate().0);
        let (client, _) = test_client().await;
        client
            .transfer_and_watch(&alice, hans.account_id(), 100_000_000_000)
            .await
            .unwrap();
        let res = client
            .transfer_and_watch(&hans, alice.account_id(), 100_000_000_000)
            .await;
        if let Err(Error::Runtime(error)) = res {
            let error2 = RuntimeError {
                module: "Balances".into(),
                error: "InsufficientBalance".into(),
            };
            assert_eq!(error, error2);
        } else {
            panic!("expected an error");
        }
    }

    #[async_std::test]
    async fn test_transfer_subscription() {
        env_logger::try_init().ok();
        let alice = PairSigner::new(AccountKeyring::Alice.pair());
        let bob = AccountKeyring::Bob.to_account_id();
        let (client, _) = test_client().await;
        let sub = client.subscribe_events().await.unwrap();
        let mut decoder = EventsDecoder::<TestRuntime>::new(client.metadata().clone());
        decoder.with_balances();
        let mut sub = EventSubscription::<TestRuntime>::new(sub, decoder);
        sub.filter_event::<TransferEvent<_>>();
        client.transfer(&alice, &bob, 10_000).await.unwrap();
        let raw = sub.next().await.unwrap().unwrap();
        let event = TransferEvent::<TestRuntime>::decode(&mut &raw.data[..]).unwrap();
        assert_eq!(
            event,
            TransferEvent {
                from: alice.account_id().clone(),
                to: bob.clone(),
                amount: 10_000,
            }
        );
    }
}
