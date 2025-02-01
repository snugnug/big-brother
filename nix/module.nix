{
  options,
  config,
  lib,
}: let
  cfg = config.modules.desktop.apps.big-brother;
  inherit (lib) mkEnableOption;
in {
  options.modules.desktop.apps.big-brother = {
    enable = lib.mkEnableOption "big-brother Nixpkgs Tracker";
    hostname = lib.mkOption {
      type = lib.types.str;
      default = "127.0.0.1";
      description = "Hostname to listen on";
    };
    port = lib.mkOption {
      type = lib.types.port;
      default = 3000;
      description = "Big-Brother web UI port";
    };
    openFirewall = lib.mkOption {
      default = false;
      type = lib.types.bool;
      description = ''
        Open the firewall from big-brother.port.
      '';
    };
    user = lib.mkOption {
      default = "big-brother";
      type = lib.types.str;
      description = "User to run big-brother under";
    };
    group = lib.mkOption {
      default = "big-brother";
      type = lib.types.str;
      description = "group to run big-brother under";
    };

    environmentFile = lib.mkOption {
      type = with lib.types; nullOr path;
      default = null;
      example = "/var/lib/secrets/big-brother.env";
      description = ''
        SystemD Environment file to use during execution.
        Example environment file.
        ```
        GITHUB_API_KEY=ghp_blahblahblahblahblahblah
        ```
      '';
    };
  };

  config = lib.mkIf cfg.enable {
    systemd.services.big-brother = {
      description = "big-brother service";
      after = ["network.target"];
      wantedBy = ["multi-user.target"];

      serviceConfig = {
        Type = "simple";
        User = cfg.user;
        Group = cfg.group;
        EnvironmentFile = lib.mkIf (cfg.environmentFile != null) [cfg.environmentFile];
        ExecStart = " /nix/store/sr1rvay66p5xczv4pfnkggnkff6rpizx-big-brother-1.0.0/bin/big-brother --host ${cfg.hostname} --port ${toString cfg.port}";

        # Hardening
        PrivateDevices = true;
        PrivateUsers = true;
        PrivateTmp = true;
        ProtectHome = true;
        ProtectKernelLogs = true;
        ProtectKernelModules = true;
        ProtectKernelTunables = true;
        RestrictAddressFamilies = [
          "AF_INET"
          "AF_INET6"
        ];
        RestrictNamespaces = true;
        RestrictRealtime = true;
        SystemCallArchitectures = "native";
      };
    };

    networking.firewall = lib.mkIf cfg.openFirewall {
      allowedTCPPorts = [cfg.port];
    };
    users.users = lib.mkIf (cfg.user == "big-brother") {
      big-brother = {
        group = cfg.group;
        description = "big-brother user";
        isSystemUser = true;
        # home = cfg.dataDir;
      };
    };
    users.groups = lib.mkIf (cfg.user == "big-brother") {
      big-brother = {};
    };
  };
}
