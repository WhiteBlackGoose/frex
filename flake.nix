{
  description = "Frex - Fractal Explorer";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  inputs.nixgl.url = "github:guibou/nixGL";
  inputs.nixgl.inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

  outputs = { nixpkgs, nixgl, ... }:
    let
      systems = [ "aarch64-darwin" "x86_64-darwin" "aarch64-linux" "x86_64-linux" ];
    in
    {
      formatter.x86_64-linux = nixpkgs.legacyPackages.x86_64-linux.nixpkgs-fmt;
      devShells = nixpkgs.lib.genAttrs systems (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          nativeBuildInputs = with pkgs; [
            cargo
            rustc
            rust-analyzer
            vscode-extensions.vadimcn.vscode-lldb
            rustfmt
            clippy

            rustPlatform.bindgenHook
            cmake
            ninja
            xorg.libX11     
            xorg.libX11.dev 
            xorg.libXi      
            xorg.libXcursor 
            xorg.libXrandr  
            xorg.libXft     
            xorg.libXft.dev 
            xorg.libXinerama
            libglvnd
            (writeScriptBin "brun" "cargo build && nixGLIntel ./target/debug/frex")
          ];
        in
        {
          default =
            pkgs.mkShell {
              inherit nativeBuildInputs;
              buildInputs = with pkgs; [
                nixgl.packages.${system}.nixGLIntel
              ];
              VSCODE_CODELLDB = "${pkgs.vscode-extensions.vadimcn.vscode-lldb}";
              LD_LIBRARY_PATH="/run/opengl-driver";
            };
        });

      packages = nixpkgs.lib.genAttrs systems (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = nixpkgs.legacyPackages.${system}.rustPlatform.buildRustPackage {
            pname = "frex";
            version = "0.0.1";
            src = ./.;

            nativeBuildInputs = [
              pkgs.installShellFiles
              pkgs.makeWrapper
            ];

            buildInputs = [
            ];

            cargoSha256 = "";
            meta = with pkgs.lib; {
              homepage = "https://github.com/WhiteBlackGoose/frex";
              description = "Fractal Explorer";
              platforms = platforms.all;
              maintainers = with maintainers; [ WhiteBlackGoose ];
              license = licenses.gpl3Plus;
              mainProgram = "frex";
            };
          };
        });
    };
}
