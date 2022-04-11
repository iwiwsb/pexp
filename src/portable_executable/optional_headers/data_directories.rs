pub struct DataDirectories {
    export: DataDir,
    import: DataDir,
    resource: DataDir,
    exception: DataDir,
    certificate: DataDir,
    base_relocation: DataDir,
    debug: DataDir,
    architecture: DataDir,
    global_ptr: DataDir,
    tls: DataDir,
    load_config: DataDir,
    bound_import: DataDir,
    iat: DataDir,
    delay_import_descriptor: DataDir,
    clr_runtime_header: DataDir,
    reserved: DataDir,
}

struct DataDir {
    virtual_address: u32,
    size: u32,
}
