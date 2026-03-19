import { Suspense } from "react";
import About from "./components/About";
import Hero from "./components/Hero";
import SearchStudents from "./components/SearchStudents";
import StudentsResult from "./components/StudentsResult";
import StudentsSearchFallback from "./components/StudentsSearchFallback";
import { siteInfoDto } from "./lib/dtos";

export default async function Home({
  searchParams,
}: {
  searchParams: Promise<{ query?: string }>;
}) {
  const { query } = await searchParams;
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
      <div className="sticky top-0">
        <About />
      </div>
      <SearchStudents query={query}>
        {query && (
          <Suspense fallback={<StudentsSearchFallback />}>
            <StudentsResult query={query} />
          </Suspense>
        )}
      </SearchStudents>
    </div>
  );
}
