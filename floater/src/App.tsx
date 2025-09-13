import { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

interface ContentPayload {
  content?: string;
  timer?: boolean;
}

function Timer() {
  const [seconds, setSeconds] = useState(0);
  const intervalRef = useRef<number | null>(null);

  useEffect(() => {
    intervalRef.current = window.setInterval(() => {
      setSeconds(prev => prev + 1);
    }, 1000);

    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
    };
  }, []);

  const minutes = Math.floor(seconds / 60);
  const remainingSeconds = seconds % 60;

  return (
    <div className="timer">
      {String(minutes).padStart(2, '0')}:{String(remainingSeconds).padStart(2, '0')}
    </div>
  );
}

function App() {
  const [content, setContent] = useState("Floater Ready");
  const [isTimer, setIsTimer] = useState(false);

  useEffect(() => {
    // Get initial content from backend
    const getInitialContent = async () => {
      try {
        const initialContent = await invoke<string>("get_content");
        if (initialContent) {
          setContent(initialContent);
        }
      } catch (error) {
        console.error("Failed to get initial content:", error);
      }
    };

    // Listen for content updates from the socket server
    const setupEventListener = async () => {
      try {
        await listen<ContentPayload>("content-updated", (event) => {
          const payload = event.payload;
          setIsTimer(payload.timer || false);
          if (payload.content && !payload.timer) {
            setContent(payload.content);
          }
        });
      } catch (error) {
        console.error("Failed to setup event listener:", error);
      }
    };

    getInitialContent();
    setupEventListener();
  }, []);

  return (
    <div className="container" data-tauri-drag-region>
      <div className="content">
        {isTimer ? <Timer /> : content}
      </div>
    </div>
  );
}

export default App;
