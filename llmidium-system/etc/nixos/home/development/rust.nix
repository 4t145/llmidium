{
  config,
  pkgs,
  ...
}: {
  home.packages = with pkgs; [
    pkgs.rustc
    pkgs.cargo
    pkgs.rustup
    pkgs.rust-analyzer
  ];
  development.rustc.enable = true;
  development.cargo.enable = true;
  development.rustup.enable = true;
  development.rust-analyzer.enable = true;
}
