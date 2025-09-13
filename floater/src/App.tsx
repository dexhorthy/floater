import { useState } from "react";
import "./App.css";

function App() {
  const [content] = useState("Floater Ready");

  return (
    <div className="container" data-tauri-drag-region>
      <div className="content">
        {content}
      </div>
    </div>
  );
}

export default App;
