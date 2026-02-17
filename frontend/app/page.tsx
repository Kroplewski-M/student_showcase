import About from "./components/About";
import Footer from "./components/Footer";
import Hero from "./components/Hero";

export default async function Home() {
  return (
    <div className="h-[100dvh] overflow-y-auto overscroll-contain snap-y snap-mandatory bg-primary text-light">
      <section className="snap-start h-[100dvh]">
        <Hero />
      </section>
      <section className="snap-start h-[100dvh] overflow-y-auto">
        <About />
      </section>
      <section className="snap-start mt-16">
        <Footer />
      </section>
    </div>
  );
}
