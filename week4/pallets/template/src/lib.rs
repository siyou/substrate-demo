#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
use sp_io::offchain_index;
use sp_runtime::offchain::storage::StorageValueRef;

const ONCHAIN_TX_KEY: &[u8] = b"my_pallet::indexing1";


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use frame_system::{
    offchain::{
        AppCrypto, CreateSignedTransaction, SendSignedTransaction,
        Signer,
    },
};

use sp_runtime::{
    offchain::{
        http, Duration,
    },
};

use serde::{Deserialize};
use sp_core::crypto::KeyTypeId;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"ocwd");
pub mod crypto {
    use super::*;
    use sp_runtime::{
        app_crypto::{app_crypto, sr25519},
        traits::Verify,
        MultiSignature, MultiSigner,
    };
    use sp_core::sr25519::Signature as Sr25519Signature;
    app_crypto!(sr25519, KEY_TYPE);

    pub struct OcwAuthId;

    impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for OcwAuthId {
        type RuntimeAppPublic = Public;
        type GenericSignature = sp_core::sr25519::Signature;
        type GenericPublic = sp_core::sr25519::Public;
    }

    impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
        for OcwAuthId
        {
            type RuntimeAppPublic = Public;
            type GenericSignature = sp_core::sr25519::Signature;
            type GenericPublic = sp_core::sr25519::Public;
        }
}


#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use sp_std::vec::Vec;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
    use core::primitive::str;

    #[derive(Deserialize, Encode, Decode, Debug)]
    struct Price {
        price_usd: u32
    }

    #[derive(Debug, Deserialize, Encode, Decode, Default)]
    pub struct IndexingData(Vec<u8>, u64);


    #[derive(Deserialize, Encode, Decode, Debug)]
    struct DotInfo {
        data: Price,
    }

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + CreateSignedTransaction<Call<Self>> {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;


    #[pallet::storage]
    pub type Prices<T: Config> = StorageValue<_, u32>;


	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}


	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}

        #[pallet::weight(0)]
        pub fn submit_data(origin: OriginFor<T>, payload: u32) -> DispatchResultWithPostInfo {

            let _who = ensure_signed(origin)?;

            // let key = Self::derived_key(frame_system::Module::<T>::block_number());
            // let data = IndexingData(b"submit_number_unsigned".to_vec(), number);
            // offchain_index::set(&key, &data.encode());

            // Read a value from storage.
			let cur = match <Prices<T>>::get() {
				None => 0,
				Some(old) => old,
			};

            <Prices<T>>::put(cur);

            log::info!("in submit_data call: {:?}", cur);

            Ok(().into())
        }


        #[pallet::weight(0)]
        pub fn extrinsic(origin: OriginFor<T>, number: u64) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let key = Self::derived_key(frame_system::Module::<T>::block_number());
            let data = IndexingData(b"submit_number_unsigned".to_vec(), number);
            offchain_index::set(&key, &data.encode());
            Ok(())
        }
	}

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {

        fn offchain_worker(block_number: T::BlockNumber) {
            log::info!("Hello World from offchain workers!: {:?}", block_number);

            if let Ok(info) = Self::fetch_dot_info() {
                log::info!("Dot Info: {:?}", info);
                let payload: u32 = info.data.price_usd;
                _ = Self::send_signed_tx(payload);

            } else {
                log::info!("Error while fetch github info!");
            }
            log::info!("Leave from offchain workers!: {:?}", block_number);

            let key = Self::derived_key(block_number);
            let storage_ref = StorageValueRef::persistent(&key);

            if let Ok(Some(data)) = storage_ref.get::<IndexingData>() {
                log::info!("local storage data: {:?}, {:?}",
                &data.0, data.1);
            } else {
                log::info!("Error reading from local storage.");
            }


        }

        fn on_initialize(_n: T::BlockNumber) -> Weight {
            log::info!("in on_initialize!");
            0
        }

        fn on_finalize(_n: T::BlockNumber) {
            log::info!("in on_finalize!");
        }

        fn on_idle(_n: T::BlockNumber, _remaining_weight: Weight) -> Weight {
            log::info!("in on_idle!");
            0
        }

    }

    impl<T: Config> Pallet<T> {

        fn send_signed_tx(payload: u32) -> Result<(), &'static str> {
            let signer = Signer::<T, T::AuthorityId>::all_accounts();
            if !signer.can_sign() {
                return Err(
                    "No local accounts available. Consider adding one via `author_insertKey` RPC.",
                    )
            }

            let results = signer.send_signed_transaction(|_account| {

                Call::submit_data { payload: payload }
            });

            for (acc, res) in &results {
                match res {
                    Ok(()) => log::info!("[{:?}] Submitted data:{:?}", acc.id, payload),
                    Err(e) => log::error!("[{:?}] Failed to submit transaction: {:?}", acc.id, e),
                }
            }

            Ok(())
        }

        fn fetch_dot_info() -> Result<DotInfo, http::Error> {
            // prepare for send request
            let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(8_000));
            let request =
                http::Request::get("https://api.coincap.io/v2/assets/polkadot");
            let pending = request
                .add_header("User-Agent", "Substrate-Offchain-Worker")
                .deadline(deadline).send().map_err(|_| http::Error::IoError)?;
            let response = pending.try_wait(deadline).map_err(|_| http::Error::DeadlineReached)??;
            if response.code != 200 {
                log::warn!("Unexpected status code: {}", response.code);
                return Err(http::Error::Unknown)
            }
            let body = response.body().collect::<Vec<u8>>();
            let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
                log::warn!("No UTF8 body");
                http::Error::Unknown
            })?;

            // parse the response str
            let dot_info: DotInfo =
                serde_json::from_str(body_str).map_err(|_| http::Error::Unknown)?;

            Ok(dot_info)
        }

        fn derived_key(block_number: T::BlockNumber) -> Vec<u8> {
            block_number.using_encoded(|encoded_bn| {
                ONCHAIN_TX_KEY.clone().into_iter()
                    .chain(b"/".into_iter())
                    .chain(encoded_bn)
                    .copied()
                    .collect::<Vec<u8>>()
            })
        }

    }

}
