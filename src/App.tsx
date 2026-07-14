import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Card from "./components/card/card";
import Collection from "./components/collection/collection";

const MOCK_CARDS: string[] = [
  "Counterspell",
  "Lightning Bolt",
  "Black Lotus",
  "Ancestral Recall",
  "Sol Ring",
];

interface CollectionSummary {
  uniqueCards: number,
  totalCards: number
};

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [summary, setSummary] = useState<CollectionSummary | null>(null);
  const [value, setValue] = useState(0);
  const [cards, setCards] = useState<ICard[]>([]);


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

  async function convertGetCardURLs(cardTitles: string[]) {
    const cardURLs = await Promise.all(cardTitles.map(title => getScryFallURL(title)));
    return cardURLs.map((url, index) => ({ title: cardTitles[index], url })) as ICard[];
  }

  async function getScryFallURL(title: string) {
    const scryfallURL = await invoke<string>("get_card_image", { name: title });
    return scryfallURL;
    // console.log(scryfallURL);
    // setCard({ title, url: scryfallURL });
  }

  useEffect(() => {
    async function fetchCards() {
      const fetchCardURLs = await convertGetCardURLs(MOCK_CARDS)
      setCards(fetchCardURLs as ICard[]);
    }
    fetchCards();
    fetchCards(); // Testing cache!
  }, [])

  return (
    <main className="container">

      <Collection cards={cards} />

      <button onClick={loadSummary}>Load Summary</button>

      {summary && (
        <section>
          <p>Unique Cards: {summary.uniqueCards}</p>
          <p>Total Cards: {summary.totalCards}</p>
        </section>
      )}

      <button onClick={increment}>Increment</button>

      <p>Current Value: {value}</p>

    </main>
  );
}

export default App;
