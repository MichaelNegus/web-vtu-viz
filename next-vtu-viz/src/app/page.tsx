import { Header, MainView, Sidetray } from "./components/layout";

export default function Home() {
  return (
    <main className="h-screen flex">
      <Sidetray />
      <div className="bg-background w-full h-screen flex flex-col">
        <Header />
        <div className="flex-1 overflow-y-scroll">
          <MainView />
        </div>
      </div>
    </main>
  );
}
