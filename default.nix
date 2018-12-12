with import <nixpkgs> {};
runCommand "dummy" {
  buildInputs = [ gcc-arm-embedded openocd ];
  shellHook = ''
    export LD_LIBRARY_PATH=$(nix-build -E 'import <nixpkgs>' -A 'gcc.cc.lib')/lib64:$LD_LIBRARY_PATH
  '';
} ""
