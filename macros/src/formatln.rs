#[macro_export]
macro_rules! formatln {
	($($arg:tt)*) => {
		format!("{}\n",format!($($arg)*))
	};
}
