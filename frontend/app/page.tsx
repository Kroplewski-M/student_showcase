import About from "./components/About";
import Hero from "./components/Hero";

export interface siteInfoDto {
  studentCount: number | null;
  projectCount: number | null;
}

export default async function Home() {
  const res = await fetch(`${process.env.API_INTERNAL_URL}/ref/site_info`);
  const data: siteInfoDto | null = await res.json();
  return (
    <div className="bg-primary text-light">
      <Hero siteInfo={data} />
      <About />
    </div>
  );
}
