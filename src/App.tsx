import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Card from "./components/card";

interface CollectionSummary {
  uniqueCards: number,
  totalCards: number
};

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [summary, setSummary] = useState<CollectionSummary | null>(null);
  const [value, setValue] = useState(0);
  const [card, setCard] = useState<ICard | null>(null);


  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  async function loadSummary() {
    const result = await invoke<CollectionSummary>("get_collection_summary");

    setSummary(result)
  }

  async function increment() {
    const result = await invoke<number>("increment");
    setValue(result)
  }

  async function getScryFallURL(title: string) {
    const scryfallURL = await invoke<string>("get_scryfall_url", { name: title });
    console.log(scryfallURL);
    setCard({ title, url: scryfallURL });
  }

  useEffect(() => {
    getScryFallURL("Counterspell");
  }, [])

  return (
    <main className="container">
      <h1>Welcome to Tauri + React</h1>
      {card && <Card card={card} />}


      <div className="row">
        <a href="https://vite.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <button onClick={loadSummary}>Load Summary</button>

      {summary && (
        <section>
          <p>Unique Cards: {summary.uniqueCards}</p>
          <p>Total Cards: {summary.totalCards}</p>
        </section>
      )}

      <button onClick={increment}>Increment</button>

      <p>Current Value: {value}</p>


      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>
    </main>
  );
}

export default App;
