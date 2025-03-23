use binding_generator::OcctPackage;

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: simple_example <OCCT SRC DIR> <PACKAGE NAME>");
        return;
    }

    let src_dir = &args[1];
    let package_name = &args[2];

    let thing = OcctPackage::new(src_dir, package_name);
}
