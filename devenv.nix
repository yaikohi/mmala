{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/basics/
  env.GREET = "devenv";

  # https://devenv.sh/packages/
  packages = [ 
    pkgs.git
    pkgs.watchexec
    pkgs.nushell
  ];

  languages.rust.enable = true;
  processes.cargo-watch.exec = "cargo-watch";

  scripts.hello.exec = ''
    echo hello from $GREET
  '';

  scripts.wr = {
    exec = ''
      watchexec -w src/ -r -e rs 'cargo run'
    '';
    description = "Runs cargo run on file changes.";
  };

  enterShell = ''
    hello
    git --version
  '';

  # https://devenv.sh/tasks/
  # tasks = {
  #   "myproj:setup".exec = "mytool build";
  #   "devenv:enterShell".after = [ "myproj:setup" ];
  # };

  # https://devenv.sh/tests/
  enterTest = ''
    echo "Running tests"
    git --version | grep --color=auto "${pkgs.git.version}"
  '';

  # https://devenv.sh/git-hooks/
  # git-hooks.hooks.shellcheck.enable = true;

  # See full reference at https://devenv.sh/reference/options/
}
