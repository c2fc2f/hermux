{
  config,
  lib,
  ...
}:
let
  cfg = config.services.hermux;
in
{
  options.services.hermux = {
    enable = lib.mkOption {
      type = lib.types.bool;
      default = false;
      description = ''
        Whether to enable Hermux.
      '';
    };

    package = lib.mkOption {
      type = lib.types.package;
      description = "The Hermux package to use";
    };

    listen = {
      address = lib.mkOption {
        type = lib.types.str;
        default = "127.0.0.1";
        description = ''
          Default network interface to listen for incoming connections. To
          listen for connections on all interfaces, use "0.0.0.0".
        '';
      };

      port = lib.mkOption {
        type = lib.types.ints.u16;
        default = 3333;
        description = ''
          The network port to listen for incoming connections.
        '';
      };
    };

    tokens = lib.mkOption {
      type = lib.types.path;
      description = ''
        Path to a file containing the list of tokens
        that the proxy will use for its own internal operations.
      '';
    };

    auth = {
      enable = lib.mkOption {
        type = lib.types.bool;
        default = false;
        description = ''
          Whether to enable Hermux auth feature.
        '';
      };

      allow = lib.mkOption {
        type = lib.types.path;
        description = ''
          Path to a file containing the list of allowed tokens.
          Incoming requests without a token from this list will be rejected.
        '';
      };
    };
  };

  config = lib.mkIf cfg.enable {
    systemd.services.hermux = {
      description = "Hermux Daemon";
      wantedBy = [ "multi-user.target" ];
      after = [ "network.target" ];

      serviceConfig = {
        ExecStart =
          "${cfg.package}/bin/hermux -a ${cfg.listen.address} -p ${toString cfg.listen.port} --tokens ${cfg.tokens}"
          + lib.optionalString cfg.auth.enable " --allow ${cfg.auth.allow}";
        User = "hermux";
        Restart = "on-failure";
      };
    };

    users.users.hermux = {
      isSystemUser = true;
      group = "hermux";
      description = "Hermux daemon user";
    };
    users.groups.hermux = { };

    environment.systemPackages = [ cfg.package ];
  };
}
