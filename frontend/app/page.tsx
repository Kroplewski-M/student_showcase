import About from "./components/About";
import Hero from "./components/Hero";

export default async function Home() {
  return (
    <div className="bg-primary text-light">
      <Hero />
      <About />
    </div>
  );
}
