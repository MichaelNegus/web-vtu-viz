import { Header, Sidetray } from "./components/layout";

export default function Home() {
  return (
    <main className="h-screen flex">
      <Sidetray />
      <div className="bg-background w-full">
        <Header />
        <div className="flex-1 p-8 bg-background">
          <div className="max-w-4xl mx-auto">
            <div className="mb-8">
              <h1 className="text-4xl font-bold text-foreground mb-4">
                Welcome to VTU Viz
              </h1>
              <p className="text-lg text-muted-foreground mb-6">
                This will be a really cool app. I promise.
              </p>
            </div>
          </div>
        </div>
      </div>
    </main>
  );
}
