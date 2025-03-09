{pkgs, ...}: {
  ##################################################################################################################
  #
  # All Ryan's Home Manager Configuration
  #
  ##################################################################################################################

  imports = [
    ../../home/core.nix
    ../../home/programs
   # ../../home/development
  ];

  programs.git = {
    userName = "llmidium";
    userEmail = "example@llmidium.com";
  };


}
