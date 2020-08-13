pub use web_sys::{
	Geolocation, Position as GeolocationPosition, PositionError as GeolocationPositionError,
	PositionOptions,
};

/// Handle to a watcher created by a watch_position() call. When dropped, the watcher will be
/// cleared.
#[derive(Debug)]
pub struct GeolocationHandle(u32);
