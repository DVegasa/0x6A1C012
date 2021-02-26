let pkgs = import <nixpkgs> {};
in pkgs.mkShell {
    name = "Rust-backend";
    buildInputs = with pkgs; [
        pkg-config
        openssl
        binutils
        gcc
        # llvmPackages.libclang
        clang
        postgresql_12
    ];
    shellHook = ''
        export LIBCLANG_PATH=${pkgs.llvmPackages.libclang}/lib
    '';
}
