{
  description = "Development environment for assembly programming and Python tools";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    devShells.${system}.default = pkgs.mkShell {
      packages = [
        pkgs.nasm
        pkgs.emu2
        pkgs.uv
        (pkgs.writeShellScriptBin "run-asm" (builtins.readFile ./run-asm.sh))
      ];
      shellHook = ''
        echo "Assembly & Python Development ENV Ready!"
        echo "-------------------------------------"
        echo "Available tools: nasm, emu2, uv"
        echo "Custom command: run-asm <file.asm>"
      '';
    };
  };
}

