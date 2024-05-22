package main

import (
	"log/slog"
	"os"
	"os/signal"
	"syscall"

	"github.com/joho/godotenv"
	"gtihub.com/nitpum/yuki/internal/bot"
)

func main() {
	slog.Info("starting...")

	err := godotenv.Load()
	if err != nil {
		slog.Error("cannot load .env file")
	}

	token := os.Getenv("DISCORD_TOKEN")
	if token == "" {
		slog.Error("required environment varaible", "key", "DISCORD_TOKEN")
		return
	}

	// if err := db.InitDatabase("./database.db"); err != nil {
	// 	slog.Error("cannot init database", "error", err)
	// 	return
	// }

	bot.Start(token)

	sc := make(chan os.Signal, 1)
	signal.Notify(sc, syscall.SIGINT, syscall.SIGTERM, os.Interrupt)
	<-sc
}
