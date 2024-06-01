{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ...}@inputs: flake-utils.lib.simpleFlake {
    inherit self nixpkgs;

    name = "word_count";
    overlay =  final: prev: {
      word_count = {
        default = final.rustPlatform.buildRustPackage {
          pname = "word_count";
          name = "word_count";

          src = ./.;
          
          cargoHash = "sha256-6MHfBpZ+I6OHMjFLtGVYokfH0tpguX1blppmrPpbWqg=";
        };
      };
    };
  };
}