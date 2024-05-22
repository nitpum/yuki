CREATE TABLE IF NOT EXISTS server_settings(
  guild_id VARCHAR(30) NOT NULL PRIMARY KEY,
  log_channel_id VARCHAR(30),
  voice_log_channel_id VARCHAR(30)
)
