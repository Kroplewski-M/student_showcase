import Footer from "./components/Footer";
import Nav from "./components/Nav";

export default async function Home() {
  return (
    <div>
      <Nav />
      <main className="min-h-screen flex flex-col items-center">
        <h1 className="text-light font-bold text-center text-4xl mt-8">
          Student Showcase 2026
        </h1>
        <div className="mt-32 mb-8 px-6 py-4 rounded-xl bg-support text-dark shadow-lg">
          <p className="font-semibold text-center">
            🚧 This page is currently under construction. Check back soon!
          </p>
        </div>
      </main>
      <Footer />
    </div>
  );
}
