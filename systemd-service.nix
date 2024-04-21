systemd.services.rs-reminder = {
  after = [ "network.target" ];
  description = "RS Reminder Bot";
  serviceConfig = {
      Type = "simple";
      DynamicUser = true;
      NoNewPrivileges = true;
      PrivateTmp = true;
      ProtectSystem = "strict";
      ProtectHome = "tmpfs";
      CapabilityBoundingSet = "";
      RestrictNamespaces = true;
      Environment = ["DISCORD_TOKEN=<token>" "DISCORD_CHANNEL_ID=<channel>"];
      SystemCallFilter = "@system-service";
      ExecStart = ''/home/mac/rs-reminder-bot/target/release/rs-reminder-bot'';
  };
};
