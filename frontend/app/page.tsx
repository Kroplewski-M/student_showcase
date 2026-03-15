import About from "./components/About";
import Hero from "./components/Hero";
import { siteInfoDto } from "./lib/dtos";

export default async function Home() {
  let data: siteInfoDto | null = null;
  try {
    const res = await fetch(`${process.env.API_INTERNAL_URL}/ref/site_info`, {
      next: {
        revalidate: 3600, //1 hour cache
      },
    });
    data = await res.json();
  } catch {}
  return (
    <div className="bg-primary text-light">
      <Hero siteInfo={data} />
      <About />
    </div>
  );
}
