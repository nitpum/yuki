package db

import (
	_ "modernc.org/sqlite"
)

func InitDatabase(dbPath string) error {
  _, err := sql.Open("sqlite", dbPath)
	if err != nil {
		return err
	}  

  return nil
}
