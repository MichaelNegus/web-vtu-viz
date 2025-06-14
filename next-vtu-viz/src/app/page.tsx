import { Header, Sidetray, MainView } from "./components/layout";
import {WasmApp} from "@/app/wasm-app/WasmApp";

export default function Home() {
  return (
    <main className="h-screen flex">
      <Sidetray />
      <div className="bg-background w-full">
        <Header />
        <MainView />
        <WasmApp />
      </div>
    </main>
  );
}
