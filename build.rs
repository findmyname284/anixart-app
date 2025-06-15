fn main() {
    glib_build_tools::compile_resources(
        &["resources"],
        "resources/resources.gresource.xml",
        "app.gresource",
    );

    println!("cargo:rerun-if-changed=resources");
    println!("cargo:rerun-if-changed=resources/resources.gresource.xml");
}
