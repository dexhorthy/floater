package main

import (
	"encoding/json"
	"fmt"
	"net"
	"os"
	"time"

	"github.com/spf13/cobra"
)

const (
	socketPath = "/tmp/floater.sock"
	version    = "1.0.0"
)

// Command represents the JSON structure sent to the socket
type Command struct {
	Action  string `json:"action"`
	Content string `json:"content,omitempty"`
	Timer   bool   `json:"timer,omitempty"`
}

// sendCommand sends a command to the floater socket and returns the response
func sendCommand(cmd Command) error {
	// Check if socket file exists first
	if _, err := os.Stat(socketPath); os.IsNotExist(err) {
		return fmt.Errorf("socket file %s not found - make sure the Floater app is running", socketPath)
	}

	// Connect to the Unix domain socket
	conn, err := net.DialTimeout("unix", socketPath, 5*time.Second)
	if err != nil {
		return fmt.Errorf("failed to connect to Floater app - make sure it's running and accepting connections")
	}
	defer conn.Close()

	// Set timeout for the entire operation
	conn.SetDeadline(time.Now().Add(10 * time.Second))

	// Marshal command to JSON
	jsonData, err := json.Marshal(cmd)
	if err != nil {
		return fmt.Errorf("failed to marshal command: %w", err)
	}

	// Send command with newline
	message := string(jsonData) + "\n"
	_, err = conn.Write([]byte(message))
	if err != nil {
		return fmt.Errorf("failed to send command: %w", err)
	}

	// Read response
	buffer := make([]byte, 1024)
	n, err := conn.Read(buffer)
	if err != nil {
		return fmt.Errorf("failed to read response: %w", err)
	}

	response := string(buffer[:n])
	fmt.Printf("Response: %s\n", response)
	return nil
}

var rootCmd = &cobra.Command{
	Use:     "floatercli",
	Short:   "CLI tool for controlling the Floater desktop application",
	Long:    `floatercli is a command-line interface for controlling the Floater desktop floating window application.`,
	Version: version,
}

var showCmd = &cobra.Command{
	Use:   "show [text]",
	Short: "Show the floater window with optional text content",
	Long: `Show the floater window with optional text content.

Examples:
  floatercli show "Hello World"
  floatercli show "Meeting in 5 minutes" --timer`,
	Args: cobra.MaximumNArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		// Get the timer flag
		timer, err := cmd.Flags().GetBool("timer")
		if err != nil {
			return fmt.Errorf("failed to get timer flag: %w", err)
		}

		// Create command
		command := Command{
			Action: "show",
			Timer:  timer,
		}

		// Add content if provided
		if len(args) > 0 {
			command.Content = args[0]
		}

		// Send command
		err = sendCommand(command)
		if err != nil {
			return err
		}

		// Success message
		msg := "✓ Floater window shown"
		if command.Content != "" {
			msg += fmt.Sprintf(` with content: "%s"`, command.Content)
		}
		if timer {
			msg += " and timer enabled"
		}
		fmt.Println(msg)

		return nil
	},
}

var hideCmd = &cobra.Command{
	Use:   "hide",
	Short: "Hide the floater window",
	Long:  `Hide the floater window from the desktop.`,
	RunE: func(cmd *cobra.Command, args []string) error {
		// Create command
		command := Command{
			Action: "hide",
		}

		// Send command
		err := sendCommand(command)
		if err != nil {
			return err
		}

		fmt.Println("✓ Floater window hidden")
		return nil
	},
}

func init() {
	// Add timer flag to show command
	showCmd.Flags().BoolP("timer", "t", false, "Enable timer widget in the floater window")

	// Add commands to root
	rootCmd.AddCommand(showCmd)
	rootCmd.AddCommand(hideCmd)

	// Set custom error handling
	rootCmd.SilenceErrors = false
	rootCmd.SilenceUsage = false
}

func main() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}
}