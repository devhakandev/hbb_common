fn main() {
    // option_env!() values in config.rs (server host/key, client profile) are baked at
    // compile time; force a rebuild of this crate when they change so switching between
    // host/admin profiles or rotating the server key actually re-bakes config.
    println!("cargo:rerun-if-env-changed=SYRD_CLIENT_PROFILE");
    println!("cargo:rerun-if-env-changed=SYRD_SERVER_HOST");
    println!("cargo:rerun-if-env-changed=SYRD_SERVER_KEY");

    let out_dir = format!("{}/protos", std::env::var("OUT_DIR").unwrap());

    std::fs::create_dir_all(&out_dir).unwrap();

    protobuf_codegen::Codegen::new()
        .pure()
        .out_dir(out_dir)
        .inputs(["protos/rendezvous.proto", "protos/message.proto"])
        .include("protos")
        .customize(protobuf_codegen::Customize::default().tokio_bytes(true))
        .run()
        .expect("Codegen failed.");
}
