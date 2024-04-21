systemd.user.services.rs-reminder = {
  enable = true;
  after = [ "network.target" ];
  description = "RS Reminder Bot";
  serviceConfig = {
      Type = "simple";
      NoNewPrivileges = true;
      PrivateTmp = true;
      ProtectSystem = "strict";
      ProtectHome = "tmpfs";
      BindReadOnlyPaths = ["/home/mac/rs-reminder-bot/target/release"];
      CapabilityBoundingSet = "";
      RestrictNamespaces = true;
      Restart = "always";
      RestartSec = "10s";
      Environment = ["DISCORD_TOKEN=<token>" "DISCORD_CHANNEL_ID=<id>"];
      SystemCallFilter = "@system-service";
      ExecStart = ''/home/mac/rs-reminder-bot/target/release/rs-reminder-bot'';
  };
};
