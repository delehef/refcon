{ pkgs, lib, config, inputs, ... }:

{
  cachix.enable = false;

  # https://devenv.sh/basics/
  env.RUST_BACKTRACE = "1";

  # https://devenv.sh/packages/
  packages = [ pkgs.git ];


  enterShell = ''***** REFCON LOADED *****'';

  # https://devenv.sh/tests/
  enterTest = '''';

  # https://devenv.sh/services/
  # services.postgres.enable = true;

  # https://devenv.sh/languages/
  languages.rust.enable = true;

  # https://devenv.sh/pre-commit-hooks/
  # pre-commit.hooks.shellcheck.enable = true;

  # https://devenv.sh/processes/
  # processes.ping.exec = "ping example.com";

  # See full reference at https://devenv.sh/reference/options/
}
