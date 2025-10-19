let
	pkgs = import <nixpkgs> {  };
	pd = builtins.toString ./.;
# Combine postgres packages into a single store
	pg = pkgs.symlinkJoin {
		name = "postgresql";
		paths = [
			pkgs.postgresql
			pkgs.postgresql17Packages.pg_cron
		];
	};
in
pkgs.mkShell {
	# Other dependencies, cli tools, etc go here.
	buildInputs = with pkgs; [
		pg
		sqlx-cli
		jq
	];

	# Postgres
	PGDATA = ".dbdata";

	shellHook = ''
		#### Utils ####
		BINDIR=${pd}/scripts
		export PATH=$PATH:$BINDIR

		GREEN='\033[0;32m'
		NC='\033[0m' # No Color

		#### Postgres ####
		# Create data directory if it does not exist and initialize it
		[ ! -d .dbdata ] && mkdir .dbdata && initdb
		
		printf "$GREEN\nUse 'start' to start Postgres server.\nUse 'stop' to stop Postgres server.\nUse 'sql' to start the Postgres cli.\n\n$NC"
		'';
}
