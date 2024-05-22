package bot

import (
	"os"

	"gopkg.in/yaml.v3"
)

type Config struct {
	ServerSettings []ServerSettings `yaml:"server_settings"`
}

type ServerSettings struct {
	GuildId           string `yaml:"guild_id"`
	LogChannelId      string `yaml:"log_channel_id"`
	VoiceLogChannelId string `yaml:"voice_log_channel_id"`
}

func readConfig() (Config, error) {
	buff, err := os.ReadFile("config.yaml")
	if err != nil {
		return Config{}, err
	}

	cfg := &Config{}
	if err := yaml.Unmarshal(buff, cfg); err != nil {
		return Config{}, err
	}

	return *cfg, nil
}

func (c Config) HasGuild(guildId string) bool {
	for _, s := range c.ServerSettings {
		if s.GuildId == guildId {
			return true
		}
	}

	return false
}

func (c Config) GetLogChannelId(guildId string) string {
	return c.GetServerSettings(guildId).LogChannelId
}

func (c Config) GetVoiceLogChannelId(guildId string) string {
	return c.GetServerSettings(guildId).VoiceLogChannelId
}

func (c Config) GetServerSettings(guildId string) ServerSettings {
	for _, s := range c.ServerSettings {
		if s.GuildId == guildId {
			return s
		}
	}

	return ServerSettings{}
}
