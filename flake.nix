{
  description = "termkata — 本地优先的术语抽讲训练工具";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      ...
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
    in
    {
      devShells = forAllSystems (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };
          lib = pkgs.lib;
          tauriLinuxLibs = with pkgs; [
            webkitgtk_4_1
            gtk3
            glib
            libsoup_3
            librsvg
            libayatana-appindicator
            xdotool
            openssl
            gsettings-desktop-schemas
            gdk-pixbuf
          ];
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              nodejs_24
              pnpm
              jujutsu
              rust-bin.stable.latest.default
            ];

            nativeBuildInputs = with pkgs; [
              pkg-config
              patchelf
              stdenv.cc
            ];

            buildInputs = lib.optionals pkgs.stdenv.isLinux tauriLinuxLibs;

            shellHook = ''
              export SSL_CERT_FILE="${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"
              export NIX_SSL_CERT_FILE="$SSL_CERT_FILE"
              ${lib.optionalString pkgs.stdenv.isLinux ''
                export XDG_DATA_DIRS="${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:''${XDG_DATA_DIRS:-/usr/local/share:/usr/share}"
              ''}
              # 部分显卡/驱动下 WebKitGTK 可能白屏，可尝试取消注释：
              # export WEBKIT_DISABLE_DMABUF_RENDERER=1
              echo "termkata dev shell  ▸ node $(node --version) / rustc $(rustc --version | cut -d' ' -f2) / jj $(jj --version | awk '{print $2}')"
            '';
          };
        }
      );
    };
}
