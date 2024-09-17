{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }@inputs : 
  let 
    pkgs = import nixpkgs { system = "x86_64-linux"; };
  in {
    devShells.x86_64-linux.default = pkgs.mkShell {
      nativeBuildInputs = with pkgs; [
        rustc 
        cargo
        openssl
        pkg-config

        cargo-leptos
        cargo-generate
        lld
        dart-sass
        binaryen
      ];

      shellHook = ''
        export CARGO_INCREMENTAL=0
        # export LEPTOS_SASS_VERSION=1.71.0
      
        cargo leptos watch
      '';
    };


    # packages.x86_64-linux.default = pkgs.callPackage ./pkg.nix {};
  };
}
