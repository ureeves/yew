//! Service to obtain device position.

use crate::Callback;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "std_web")] {
	mod std_web;
	pub use self::std_web::*;
    } else if #[cfg(feature = "web_sys")] {
	mod web_sys;
	pub use self::web_sys::*;
    }
}

/// Service to retrieve device position.
#[derive(Debug)]
pub struct GeolocationService {
	geolocation: Geolocation,
}

impl GeolocationService {
	/// Creates a new geolocation service.
	pub fn new() -> Result<Self, &'static str> {
		unimplemented!()
	}

	/// Used to get the current position of the device.
	pub fn get_current_position(
		_success: Callback<GeolocationPosition>,
		_error: Option<Callback<GeolocationPositionError>>,
		_options: Option<PositionOptions>,
	) {
		unimplemented!()
	}

	/// Register a handler function to be called automatically each time the position of the
	/// device changes.
	pub fn watch_position(
		_success: Callback<GeolocationPosition>,
		_error: Option<Callback<GeolocationPositionError>>,
		_options: Option<PositionOptions>,
	) -> GeolocationHandle {
		unimplemented!()
	}
}
