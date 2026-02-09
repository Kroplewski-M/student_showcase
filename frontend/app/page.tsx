import Hero from "./components/Hero";
import Nav from "./components/Nav";

export default async function Home() {
  return (
    <div className="bg-primary text-light">
      <Nav />
      <Hero />

      <main className="relative z-10 bg-primary">
        <section className="min-h-screen flex flex-col items-center justify-center px-6">
          <div className="mt-32 mb-8 px-6 py-4 rounded-xl bg-support text-dark shadow-lg">
            <p className="font-semibold text-center">
              🚧 This page is currently under construction. Check back soon!
            </p>
          </div>
        </section>
      </main>
    </div>
  );
}
