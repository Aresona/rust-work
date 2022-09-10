#![cfg_attr(not(feature = "std"), no_std)]

// extern crate core;
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use codec::{Decode, Encode, MaxEncodedLen};
	use frame_support::{
		pallet_prelude::*,
		traits::{Currency, Randomness, ReservableCurrency},
	};
	use frame_system::pallet_prelude::*;
	use sp_io::hashing::blake2_128;
	use sp_runtime::traits::{AtLeast32BitUnsigned, Bounded, One};

	// type KittyIndex = u32;
	//
	// #[pallet::type_value]
	// pub fn GetDefaultValue() -> KittyIndex {
	// 	0_u32
	// }

	#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
	pub struct Kitty(pub [u8; 16]);

	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;

		type Currency: ReservableCurrency<Self::AccountId>;

		// Max value of Ownered Kitties
		#[pallet::constant]
		type MaxLength: Get<u32>;

		#[pallet::constant]
		type ReservableFee: Get<BalanceOf<Self>>;

		type KittyIndex: MaxEncodedLen
			+ Parameter
			+ Member
			+ Default
			+ Bounded
			+ Copy
			+ AtLeast32BitUnsigned;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn next_kitty_id)]
	pub type NextKittyId<T: Config> = StorageValue<_, T::KittyIndex, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T: Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, Kitty>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_owner)]
	pub type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, T::AccountId>;

	#[pallet::storage]
	#[pallet::getter(fn ownered_kitties)]
	pub type OwneredKitties<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<Kitty, T::MaxLength>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		KittyCreated(T::AccountId, T::KittyIndex, Kitty),
		KittyBreed(T::AccountId, T::KittyIndex, Kitty),
		KittyTransferred(T::AccountId, T::AccountId, T::KittyIndex),
	}

	#[pallet::error]
	pub enum Error<T> {
		InvalidKittyId,
		NotOwner,
		SameKittyId,
		OwneredKittiesExceed,
		OwneredKittiesNotTransferred,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn create(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let kitty_id = Self::get_next_id().map_err(|_| Error::<T>::InvalidKittyId)?;

            // create kitty
			let dna = Self::random_value(&who);
			let kitty = Kitty(dna);

			// reserve token
			T::Currency::reserve(&who, T::ReservableFee::get())?;

            // store
			Kitties::<T>::insert(kitty_id, &kitty);
			KittyOwner::<T>::insert(kitty_id, &who);
			NextKittyId::<T>::set(kitty_id + One::one());
			OwneredKitties::<T>::try_mutate(&who, |kitties_vec| {
				kitties_vec.try_push(kitty.clone())
			})
			.map_err(|_| <Error<T>>::OwneredKittiesExceed)?;

			// Emit an event
			Self::deposit_event(Event::KittyCreated(who, kitty_id, kitty));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn breed(
			origin: OriginFor<T>,
			kitty_id_1: T::KittyIndex,
			kitty_id_2: T::KittyIndex,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// check kitty id
			ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameKittyId);
			let kitty_1 = Self::get_kitty(kitty_id_1).map_err(|_| Error::<T>::InvalidKittyId)?;
			let kitty_2 = Self::get_kitty(kitty_id_2).map_err(|_| Error::<T>::InvalidKittyId)?;

			// get next id
			let kitty_id = Self::get_next_id().map_err(|_| Error::<T>::InvalidKittyId)?;

			// selector for breeding
			let selector = Self::random_value(&who);

			let mut data = [0u8; 16];
			for i in 0..kitty_1.0.len() {
				// 0 choose kitty2, and 1 choose kitty1
				data[i] = (kitty_1.0[i] & selector[i]) | (kitty_2.0[i] & !selector[i]);
			}

			let new_kitty = Kitty(data);

			// reserve token
			T::Currency::reserve(&who, T::ReservableFee::get())?;

            // store
			<Kitties<T>>::insert(kitty_id, &new_kitty);
			KittyOwner::<T>::insert(kitty_id, &who);
			NextKittyId::<T>::set(kitty_id + One::one());
			OwneredKitties::<T>::try_mutate(&who, |kitties_vec| {
				kitties_vec.try_push(new_kitty.clone())
			})
			.map_err(|_| <Error<T>>::OwneredKittiesExceed)?;

            // emit event
			Self::deposit_event(Event::KittyBreed(who, kitty_id, new_kitty));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn transfer(
			origin: OriginFor<T>,
			kitty_id: T::KittyIndex,
			new_owner: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let origin_kitty = Self::get_kitty(kitty_id).map_err(|_| Error::<T>::InvalidKittyId)?;

			// transfer reserve
			T::Currency::reserve(&new_owner, T::ReservableFee::get())?;
			T::Currency::unreserve(&who, T::ReservableFee::get());

			ensure!(Self::kitty_owner(kitty_id) == Some(who.clone()), Error::<T>::NotOwner);

            // remove kitty from origin
			<KittyOwner<T>>::insert(kitty_id, &new_owner);
			OwneredKitties::<T>::try_mutate(&who, |kitties_vec| {
				if let Some(index) = kitties_vec.iter().position(|kitty| kitty == &origin_kitty) {
					kitties_vec.swap_remove(index);
					return Ok(());
				}
				Err(())
			})
			.map_err(|_| Error::<T>::OwneredKittiesNotTransferred)?;

            // add kitty to new_owner
			OwneredKitties::<T>::try_mutate(&new_owner, |kitties_vec| {
				kitties_vec.try_push(origin_kitty.clone())
			})
			.map_err(|_| <Error<T>>::OwneredKittiesExceed)?;

            // emit event
			Self::deposit_event(Event::KittyTransferred(who, new_owner, kitty_id));

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		// get a random 256
		fn random_value(sender: &T::AccountId) -> [u8; 16] {
			let payload = (
				T::Randomness::random_seed(),
				&sender,
				<frame_system::Pallet<T>>::extrinsic_index(),
			);

			payload.using_encoded(blake2_128)
		}

		// get next id
		fn get_next_id() -> Result<T::KittyIndex, ()> {
			match Self::next_kitty_id() {
				val if val == T::KittyIndex::max_value() => Err(()),
				val => Ok(val),
			}
		}

		// get kitty via id
		fn get_kitty(kitty_id: T::KittyIndex) -> Result<Kitty, ()> {
			match Self::kitties(kitty_id) {
				Some(kitty) => Ok(kitty),
				None => Err(()),
			}
		}
	}
}
