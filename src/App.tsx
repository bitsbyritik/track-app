import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [taskName, setTaskName] = useState("");

  async function startTracking() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    if (!taskName.trim()) {
      console.error("Task name cannot be empty.");
      return;
    }
    try {
      await invoke("set_task_name", { task: taskName });
      await invoke("start_tracking");
    } catch (err) {
      console.error("Error setting up task name: ", err);
    }
  }

  return (
    <main className="container">
      <h1>Welcome to Track Event</h1>

      <form className="row" onClick={startTracking}>
        <input
          id="greet-input"
          onChange={(e) => setTaskName(e.currentTarget.value)}
          placeholder="Enter a task name..."
        />
        <button type="submit">Start Tracking</button>
      </form>
    </main>
  );
}

export default App;
