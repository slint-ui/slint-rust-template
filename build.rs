fn main() {
    slint_build::compile("ui/appwindow.slint").unwrap();
	println!("cargo:rerun-if-changed=ui/appwindow.slint");
}
