import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import MyApp from "hello_react_library";

function App() {
  const [greetMessage, setGreetMessage] = useState("");
  const [name, _setName] = useState("");

  async function greet(str: string) {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMessage(await invoke("greet", { str }));
  }

  return (
    <main className="container">
      <MyApp greetMessage={greetMessage} setter={(e) => greet(e)} />
    </main>
  );
}

export default App;
