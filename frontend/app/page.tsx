import About from "./components/About";
import Hero from "./components/Hero";

export default async function Home() {
  return (
    <div className="h-screen overflow-y-auto snap-y snap-mandatory bg-primary text-light">
      <section className="snap-start">
        <Hero />
      </section>
      <main className="relative z-10">
        <section className="snap-start">
          <About />
        </section>
      </main>
    </div>
  );
}
