import Link from "next/link";
import StudentLookup from "./StudentLookup";
import Dashboard from "./Dashboard";
import { DashboardData } from "./Dashboard";
import { redirect } from "next/navigation";
import { authFetch } from "../lib/auth";

export default async function Admin({
  searchParams,
}: {
  searchParams: Promise<{ view?: string }>;
}) {
  const { view } = await searchParams;
  const isStudentLookup = view === "student-lookup";

  let dashboardData: DashboardData | null = null;

  if (!isStudentLookup) {
    try {
      const res = await authFetch(
        `${process.env.API_INTERNAL_URL}/admin/dashboard`,
        { cache: "no-store" },
      );
      if (!res.ok) {
        redirect("/admin?view=student-lookup");
      }
      dashboardData = await res.json();
    } catch {
      redirect("/admin?view=student-lookup");
    }
  }

  const btnClass =
    "cursor-pointer gap-2 rounded-lg border border-secondary/20 bg-secondary/6 px-4 py-2 text-sm font-medium text-secondary/70 transition-all hover:border-secondary/35 hover:bg-secondary/10 hover:text-secondary";

  return (
    <main className="mx-auto px-5 min-h-screen">
      <div className="flex items-center justify-center gap-2 mb-5 mt-32">
        <Link href="/admin" className={btnClass}>
          Dashboard
        </Link>
        <Link href="/admin?view=student-lookup" className={btnClass}>
          Student Lookup
        </Link>
      </div>

      <div className="flex items-center justify-center">
        {isStudentLookup ? (
          <StudentLookup />
        ) : (
          dashboardData && <Dashboard data={dashboardData} />
        )}
      </div>
    </main>
  );
}
