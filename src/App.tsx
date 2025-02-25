import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [taskName, setTaskName] = useState("");
  const [isTracking, setIsTracking] = useState<boolean>(false);

  async function startTracking() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    if (!taskName.trim()) {
      console.error("Task name cannot be empty.");
      return;
    }
    try {
      await invoke("set_task_name", { task: taskName });
      await invoke("start_tracking");
      setIsTracking(true);
    } catch (err) {
      console.error("Error setting up task name: ", err);
    }
  }

  async function stopTracking() {
    try {
      await invoke("stop_tracking");
      setIsTracking(false);
    } catch (err) {
      console.error("Error during stop:", err);
    }
  }

  return (
    <main className="container">
      <h1>Welcome to Track Event</h1>

      <div className="row">
        {!isTracking ? (
          <div>
            <input
              id="greet-input"
              onChange={(e) => setTaskName(e.currentTarget.value)}
              placeholder="Enter a task name..."
            />
            <button onClick={startTracking}>Start Tracking</button>
          </div>
        ) : (
          <div id="greet-input">
            <div>{taskName} is being tracked...</div>
            <button onClick={stopTracking}>Stop Tracking</button>
          </div>
        )}
      </div>
    </main>
  );
}

export default App;
