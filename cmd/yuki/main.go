package main

import (
	"log/slog"
	"os"
	"os/signal"
	"syscall"

	"github.com/joho/godotenv"
	"gtihub.com/nitpum/yuki/intenral/bot"
	"gtihub.com/nitpum/yuki/internal/db"
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

  db.InitDatabase("database.db")

	bot.Start(token)

	sc := make(chan os.Signal, 1)
	signal.Notify(sc, syscall.SIGINT, syscall.SIGTERM, os.Interrupt)
	<-sc
}

