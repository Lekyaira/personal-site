let
	pkgs = import <nixpkgs> {};
	pd = builtins.toString ./.;
in
pkgs.mkShell {
	buildInputs = with pkgs; [
		nodejs_24
	];
}
