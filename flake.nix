{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          # pkg-config
          # openssl
        ];

        LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
          vulkan-loader
          wayland
          libxkbcommon

          # fontconfig
          # libGL
        ];
      };
    };
}