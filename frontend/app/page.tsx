import About from "./components/About";
import Hero from "./components/Hero";

export default async function Home() {
  return (
    <div className="relative bg-primary text-light">
      <div className="h-[100dvh] overflow-y-auto snap-y snap-mandatory">
        <section className="snap-start min-h-[100dvh]">
          <Hero />
        </section>
        <section className="snap-start min-h-[100dvh]">
          <About />
        </section>
      </div>
    </div>
  );
}
