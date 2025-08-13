# flake.nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      stdenv = pkgs.stdenv;
      lib = pkgs.lib;
      fhs = pkgs.buildFHSEnv {
        name = "fhs-shell";
        targetPkgs = pkgs: with pkgs; [gcc glibc clang boringssl boringssl.dev nghttp2 nghttp3 nghttp3.dev ngtcp2];
      };
    in
      {
        devShells.${system}.default = pkgs.mkShell {
            inputsFrom = [ fhs.env ];
            shellHook = ''
                export BINDGEN_EXTRA_CLANG_ARGS="\
                  $(< ${pkgs.clang}/nix-support/cc-cflags) \
                  -I${pkgs.glibc.dev}/include \
                  -I${pkgs.boringssl.dev}/include \
                "
                export CFLAGS="-I${pkgs.boringssl.dev}/include -I${pkgs.glibc.dev}/include -O2"
                export LIBCLANG_PATH="${pkgs.libclang.lib}/lib"
            '';
        };
      };
}