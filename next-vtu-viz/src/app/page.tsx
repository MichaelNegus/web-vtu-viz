import {Header, MainView, Sidetray} from "./components/layout";

export default function Home() {
  return (
    <main className="h-screen flex">
      <Sidetray />
      <div className="bg-background w-full">
        <Header />
        <MainView />
      </div>
    </main>
  );
}
