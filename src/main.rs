use std::env::temp_dir;

fn main() {
	let tmp_dir = temp_dir();
	println!("tmp folder: {:?}", tmp_dir);
}
