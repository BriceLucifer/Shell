package main

import (
        "fmt"
        "github.com/chzyer/readline"
        "io"
        "os"
        "os/exec"
        "os/signal"
        "strings"
)

func exit() {
        fmt.Println("Exiting shell...")
        os.Exit(0)
}

func launch(args []string) (int, string) {
        if len(args) == 0 {
                return 1, "No command provided."
        }

        // Handle `cd` command
        if args[0] == "cd" {
                if len(args) < 2 {
                        return 1, "cd: missing argument"
                }
                err := os.Chdir(args[1])
                if err != nil {
                        return 1, fmt.Sprintf("cd: %v", err)
                }
                return 0, "" // Successfully changed directory
        }

        cmd := exec.Command(args[0], args[1:]...)
        cmd.Stdin = os.Stdin
        cmd.Stdout = os.Stdout
        cmd.Stderr = os.Stderr
        err := cmd.Run()
        if err != nil {
                return 1, fmt.Sprintf("shell error: %v", err)
        }
        return 0, ""
}

func loop() {
        config := &readline.Config{
                Prompt:      "🌌> ",
                HistoryFile: "./shell_history",
        }
        rl, err := readline.NewEx(config)
        if err != nil {
                fmt.Printf("Failed to create readline instance: %v\n", err)
                return
        }
        defer func(rl *readline.Instance) {
                err := rl.Close()
                if err != nil {
                        fmt.Printf("Error closing \n")
                }
        }(rl)

        // Enable command history
        rl.HistoryEnable()

        for {
                input, err := rl.Readline()
                if err != nil {
                        if err == io.EOF {
                                break // Exit on EOF (Ctrl+D)
                        }
                        fmt.Printf("Error reading input: %v\n", err)
                        continue
                }

                // Process input
                input = strings.TrimSpace(input)
                if input == "" {
                        continue
                }

                if input == "exit" {
                        exit()
                }

                args := strings.Fields(input)
                status, output := launch(args)
                if status != 0 {
                        fmt.Printf("Command failed: %s\n", output)
                } else {
                        fmt.Print(output)
                }

                // Save command to history
                if err := rl.SaveHistory(input); err != nil {
                        fmt.Printf("Failed to save command to history: %v\n", err)
                }
        }
}

func main() {
        // Capture interrupt signals for graceful exit
        c := make(chan os.Signal, 1)
        signal.Notify(c, os.Interrupt)
        go func() {
                for range c {
                        fmt.Println("\nProgram exited.")
                        os.Exit(0)
                }
        }()

        // Start the command loop
        loop()
}