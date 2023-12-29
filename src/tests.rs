#[cfg(test)]
mod tests {
	use crate::commands;
	
	
	#[test]
	fn test_init() {
	
		commands::init();
	
		assert_eq!(1, 1);
	}
}