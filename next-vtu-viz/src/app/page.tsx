import { Canvas, EmptyState } from "./components/views";

export default function Home() {
  return (
    <div className="flex-1 overflow-y-scroll">
      <EmptyState />
      <Canvas />
    </div>
  );
}
