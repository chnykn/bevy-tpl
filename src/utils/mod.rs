pub mod pipe;
pub mod trait_extension;
pub mod color;


// ?1, ?2 ... ?n
pub fn gen_placeholders(n: usize) -> String {
	let mut result = String::new();
	for i in 1..=n {
		result.push_str(&format!("?{}", i));
		if i < n {
			result.push_str(", ");
		}
	}
	result
}