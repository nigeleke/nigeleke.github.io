{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/master";
  };


  outputs = { nixpkgs, systems, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = [ pkgs.zola pkgs.vscode ];
      };
    };
}
