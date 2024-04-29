package bot

import (
	"log/slog"

	"github.com/bwmarrin/discordgo"
)

func Start(token string) {
	client, err := discordgo.New("Bot " + token)
	if err != nil {
		slog.Error("cannot create Discord session", "error", err)
		return
	}
	defer client.Close()

	client.AddHandler(handleVoiceActivity)

	if err := client.Open(); err != nil {
		slog.Error("cannot open connection to Discord", "error", err)
		return
	}

	slog.Info("bot is now running...")
}

func handleVoiceActivity(session *discordgo.Session, event *discordgo.VoiceStateUpdate) {
	slog.Info("voice activity", "event", event)

}
